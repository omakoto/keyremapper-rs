#!/usr/bin/python3

import sys
import os
from typing import Iterable, Dict
import re
import textwrap

SCRIPT_PATH = os.path.dirname(os.path.realpath(__file__))

OUT_DIR = SCRIPT_PATH + "/../src/evdev/"

INPUT_EVENT_CODES_H = "/usr/include/linux/input-event-codes.h"
INPUT_EVENT_CODES_H_OUT = OUT_DIR + "linux_input-event-codes.h.rs"


def get_event_codes(in_file: str) -> Dict[str, str]:
	content = open(in_file).read()

	# Remove comments and continuation lines.
	content = re.sub(r'''( // .*? \n | /\* .*? \*/ | \\\n )''', '', content, 0, re.VERBOSE + re.DOTALL + re.MULTILINE)

	ret = {}
	for line in content.split('\n'):
		m = re.match(r'''^\# \s* define \s+ (\S+) \s+ ( \S .*? )$''', line, re.VERBOSE)
		if not m:
			continue
		name, value = m.groups()

		# Remove trailing spaces
		value = re.sub(r'''\s+$''', '', value)

		# Remove wrapping parens.
		value = re.sub(r'''^\((.*)\)$''', '\\1', value)

		ret[name] = value

	return ret

def generate_event_codes(in_file: str, out_file: str) -> None:
	event_codes = get_event_codes(in_file)

	with open(out_file, 'w') as out:
		out.write(textwrap.dedent(f'''\
			// Generated from {in_file}
			
			'''))

		# All the constants.
		for key, value in event_codes.items():
			out.write(f'''pub const {key} : i32 = {value};\n''')

		# The ALL_KEYS array.
		out.write('\n')
		out.write('''pub static ALL_KEYS: &'static [i32] = &[\n''')
		for key in event_codes.keys():
			if re.search(r'''(_CNT|_MAX)$''', key):
				continue
			if not re.search(r'''^(KEY_|BTN_)''', key):
				continue
			out.write(f'''        {key},\n''')
		out.write('''        ];\n''')

		# Generate a function that converts event types and codes to strings.
		prefixes = ["SYN", "KEY", "REL", "ABS", "MSC", "SW", "LED", "SND", "REP"]

		# EV_*
		out.write(textwrap.dedent('''
			pub fn get_type_name(type_: i32) -> &'static str {
				match type_ {
			'''))
		for type in ("EV_" + type for type in prefixes):
			out.write('''		''' + type + ''' => "''' + type + '''",\n''')
		out.write(textwrap.dedent('''\
					_ => "",
				}
			}
			'''))

		# Event codes
		out.write(textwrap.dedent('''
			#[allow(unreachable_patterns)] // Needed because of aliases.
			pub fn get_code_name(type_: i32, code: i32) -> &'static str {
				match type_ {
			'''))
		for prefix in prefixes:
			if prefix == "EV":
				continue
			out.write('''		EV_''' + prefix + ''' => match code {\n''')

			for key in event_codes.keys():
				if re.search(r'''(_CNT|_MAX)$''', key):
					continue
				if key.startswith(prefix + "_") or (prefix == "KEY" and key.startswith("BTN_")):
					out.write(f'''			{key} => "{key}",\n''')

			out.write('''			_ => "",\n''')
			out.write('''		}\n''')
		out.write(textwrap.dedent('''\
					_ => "",
				}
			}
			'''))


def main(args: Iterable[str]) -> None:
	generate_event_codes(INPUT_EVENT_CODES_H, INPUT_EVENT_CODES_H_OUT)

if __name__ == "__main__":
	main(sys.argv[1:])	