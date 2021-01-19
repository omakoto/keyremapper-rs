// Generated from /usr/include/linux/input-event-codes.h

pub const INPUT_PROP_POINTER : i32 = 0x00;
pub const INPUT_PROP_DIRECT : i32 = 0x01;
pub const INPUT_PROP_BUTTONPAD : i32 = 0x02;
pub const INPUT_PROP_SEMI_MT : i32 = 0x03;
pub const INPUT_PROP_TOPBUTTONPAD : i32 = 0x04;
pub const INPUT_PROP_POINTING_STICK : i32 = 0x05;
pub const INPUT_PROP_ACCELEROMETER : i32 = 0x06;
pub const INPUT_PROP_MAX : i32 = 0x1f;
pub const INPUT_PROP_CNT : i32 = INPUT_PROP_MAX + 1;
pub const EV_SYN : i32 = 0x00;
pub const EV_KEY : i32 = 0x01;
pub const EV_REL : i32 = 0x02;
pub const EV_ABS : i32 = 0x03;
pub const EV_MSC : i32 = 0x04;
pub const EV_SW : i32 = 0x05;
pub const EV_LED : i32 = 0x11;
pub const EV_SND : i32 = 0x12;
pub const EV_REP : i32 = 0x14;
pub const EV_FF : i32 = 0x15;
pub const EV_PWR : i32 = 0x16;
pub const EV_FF_STATUS : i32 = 0x17;
pub const EV_MAX : i32 = 0x1f;
pub const EV_CNT : i32 = EV_MAX+1;
pub const SYN_REPORT : i32 = 0;
pub const SYN_CONFIG : i32 = 1;
pub const SYN_MT_REPORT : i32 = 2;
pub const SYN_DROPPED : i32 = 3;
pub const SYN_MAX : i32 = 0xf;
pub const SYN_CNT : i32 = SYN_MAX+1;
pub const KEY_RESERVED : i32 = 0;
pub const KEY_ESC : i32 = 1;
pub const KEY_1 : i32 = 2;
pub const KEY_2 : i32 = 3;
pub const KEY_3 : i32 = 4;
pub const KEY_4 : i32 = 5;
pub const KEY_5 : i32 = 6;
pub const KEY_6 : i32 = 7;
pub const KEY_7 : i32 = 8;
pub const KEY_8 : i32 = 9;
pub const KEY_9 : i32 = 10;
pub const KEY_0 : i32 = 11;
pub const KEY_MINUS : i32 = 12;
pub const KEY_EQUAL : i32 = 13;
pub const KEY_BACKSPACE : i32 = 14;
pub const KEY_TAB : i32 = 15;
pub const KEY_Q : i32 = 16;
pub const KEY_W : i32 = 17;
pub const KEY_E : i32 = 18;
pub const KEY_R : i32 = 19;
pub const KEY_T : i32 = 20;
pub const KEY_Y : i32 = 21;
pub const KEY_U : i32 = 22;
pub const KEY_I : i32 = 23;
pub const KEY_O : i32 = 24;
pub const KEY_P : i32 = 25;
pub const KEY_LEFTBRACE : i32 = 26;
pub const KEY_RIGHTBRACE : i32 = 27;
pub const KEY_ENTER : i32 = 28;
pub const KEY_LEFTCTRL : i32 = 29;
pub const KEY_A : i32 = 30;
pub const KEY_S : i32 = 31;
pub const KEY_D : i32 = 32;
pub const KEY_F : i32 = 33;
pub const KEY_G : i32 = 34;
pub const KEY_H : i32 = 35;
pub const KEY_J : i32 = 36;
pub const KEY_K : i32 = 37;
pub const KEY_L : i32 = 38;
pub const KEY_SEMICOLON : i32 = 39;
pub const KEY_APOSTROPHE : i32 = 40;
pub const KEY_GRAVE : i32 = 41;
pub const KEY_LEFTSHIFT : i32 = 42;
pub const KEY_BACKSLASH : i32 = 43;
pub const KEY_Z : i32 = 44;
pub const KEY_X : i32 = 45;
pub const KEY_C : i32 = 46;
pub const KEY_V : i32 = 47;
pub const KEY_B : i32 = 48;
pub const KEY_N : i32 = 49;
pub const KEY_M : i32 = 50;
pub const KEY_COMMA : i32 = 51;
pub const KEY_DOT : i32 = 52;
pub const KEY_SLASH : i32 = 53;
pub const KEY_RIGHTSHIFT : i32 = 54;
pub const KEY_KPASTERISK : i32 = 55;
pub const KEY_LEFTALT : i32 = 56;
pub const KEY_SPACE : i32 = 57;
pub const KEY_CAPSLOCK : i32 = 58;
pub const KEY_F1 : i32 = 59;
pub const KEY_F2 : i32 = 60;
pub const KEY_F3 : i32 = 61;
pub const KEY_F4 : i32 = 62;
pub const KEY_F5 : i32 = 63;
pub const KEY_F6 : i32 = 64;
pub const KEY_F7 : i32 = 65;
pub const KEY_F8 : i32 = 66;
pub const KEY_F9 : i32 = 67;
pub const KEY_F10 : i32 = 68;
pub const KEY_NUMLOCK : i32 = 69;
pub const KEY_SCROLLLOCK : i32 = 70;
pub const KEY_KP7 : i32 = 71;
pub const KEY_KP8 : i32 = 72;
pub const KEY_KP9 : i32 = 73;
pub const KEY_KPMINUS : i32 = 74;
pub const KEY_KP4 : i32 = 75;
pub const KEY_KP5 : i32 = 76;
pub const KEY_KP6 : i32 = 77;
pub const KEY_KPPLUS : i32 = 78;
pub const KEY_KP1 : i32 = 79;
pub const KEY_KP2 : i32 = 80;
pub const KEY_KP3 : i32 = 81;
pub const KEY_KP0 : i32 = 82;
pub const KEY_KPDOT : i32 = 83;
pub const KEY_ZENKAKUHANKAKU : i32 = 85;
pub const KEY_102ND : i32 = 86;
pub const KEY_F11 : i32 = 87;
pub const KEY_F12 : i32 = 88;
pub const KEY_RO : i32 = 89;
pub const KEY_KATAKANA : i32 = 90;
pub const KEY_HIRAGANA : i32 = 91;
pub const KEY_HENKAN : i32 = 92;
pub const KEY_KATAKANAHIRAGANA : i32 = 93;
pub const KEY_MUHENKAN : i32 = 94;
pub const KEY_KPJPCOMMA : i32 = 95;
pub const KEY_KPENTER : i32 = 96;
pub const KEY_RIGHTCTRL : i32 = 97;
pub const KEY_KPSLASH : i32 = 98;
pub const KEY_SYSRQ : i32 = 99;
pub const KEY_RIGHTALT : i32 = 100;
pub const KEY_LINEFEED : i32 = 101;
pub const KEY_HOME : i32 = 102;
pub const KEY_UP : i32 = 103;
pub const KEY_PAGEUP : i32 = 104;
pub const KEY_LEFT : i32 = 105;
pub const KEY_RIGHT : i32 = 106;
pub const KEY_END : i32 = 107;
pub const KEY_DOWN : i32 = 108;
pub const KEY_PAGEDOWN : i32 = 109;
pub const KEY_INSERT : i32 = 110;
pub const KEY_DELETE : i32 = 111;
pub const KEY_MACRO : i32 = 112;
pub const KEY_MUTE : i32 = 113;
pub const KEY_VOLUMEDOWN : i32 = 114;
pub const KEY_VOLUMEUP : i32 = 115;
pub const KEY_POWER : i32 = 116;
pub const KEY_KPEQUAL : i32 = 117;
pub const KEY_KPPLUSMINUS : i32 = 118;
pub const KEY_PAUSE : i32 = 119;
pub const KEY_SCALE : i32 = 120;
pub const KEY_KPCOMMA : i32 = 121;
pub const KEY_HANGEUL : i32 = 122;
pub const KEY_HANGUEL : i32 = KEY_HANGEUL;
pub const KEY_HANJA : i32 = 123;
pub const KEY_YEN : i32 = 124;
pub const KEY_LEFTMETA : i32 = 125;
pub const KEY_RIGHTMETA : i32 = 126;
pub const KEY_COMPOSE : i32 = 127;
pub const KEY_STOP : i32 = 128;
pub const KEY_AGAIN : i32 = 129;
pub const KEY_PROPS : i32 = 130;
pub const KEY_UNDO : i32 = 131;
pub const KEY_FRONT : i32 = 132;
pub const KEY_COPY : i32 = 133;
pub const KEY_OPEN : i32 = 134;
pub const KEY_PASTE : i32 = 135;
pub const KEY_FIND : i32 = 136;
pub const KEY_CUT : i32 = 137;
pub const KEY_HELP : i32 = 138;
pub const KEY_MENU : i32 = 139;
pub const KEY_CALC : i32 = 140;
pub const KEY_SETUP : i32 = 141;
pub const KEY_SLEEP : i32 = 142;
pub const KEY_WAKEUP : i32 = 143;
pub const KEY_FILE : i32 = 144;
pub const KEY_SENDFILE : i32 = 145;
pub const KEY_DELETEFILE : i32 = 146;
pub const KEY_XFER : i32 = 147;
pub const KEY_PROG1 : i32 = 148;
pub const KEY_PROG2 : i32 = 149;
pub const KEY_WWW : i32 = 150;
pub const KEY_MSDOS : i32 = 151;
pub const KEY_COFFEE : i32 = 152;
pub const KEY_SCREENLOCK : i32 = KEY_COFFEE;
pub const KEY_ROTATE_DISPLAY : i32 = 153;
pub const KEY_DIRECTION : i32 = KEY_ROTATE_DISPLAY;
pub const KEY_CYCLEWINDOWS : i32 = 154;
pub const KEY_MAIL : i32 = 155;
pub const KEY_BOOKMARKS : i32 = 156;
pub const KEY_COMPUTER : i32 = 157;
pub const KEY_BACK : i32 = 158;
pub const KEY_FORWARD : i32 = 159;
pub const KEY_CLOSECD : i32 = 160;
pub const KEY_EJECTCD : i32 = 161;
pub const KEY_EJECTCLOSECD : i32 = 162;
pub const KEY_NEXTSONG : i32 = 163;
pub const KEY_PLAYPAUSE : i32 = 164;
pub const KEY_PREVIOUSSONG : i32 = 165;
pub const KEY_STOPCD : i32 = 166;
pub const KEY_RECORD : i32 = 167;
pub const KEY_REWIND : i32 = 168;
pub const KEY_PHONE : i32 = 169;
pub const KEY_ISO : i32 = 170;
pub const KEY_CONFIG : i32 = 171;
pub const KEY_HOMEPAGE : i32 = 172;
pub const KEY_REFRESH : i32 = 173;
pub const KEY_EXIT : i32 = 174;
pub const KEY_MOVE : i32 = 175;
pub const KEY_EDIT : i32 = 176;
pub const KEY_SCROLLUP : i32 = 177;
pub const KEY_SCROLLDOWN : i32 = 178;
pub const KEY_KPLEFTPAREN : i32 = 179;
pub const KEY_KPRIGHTPAREN : i32 = 180;
pub const KEY_NEW : i32 = 181;
pub const KEY_REDO : i32 = 182;
pub const KEY_F13 : i32 = 183;
pub const KEY_F14 : i32 = 184;
pub const KEY_F15 : i32 = 185;
pub const KEY_F16 : i32 = 186;
pub const KEY_F17 : i32 = 187;
pub const KEY_F18 : i32 = 188;
pub const KEY_F19 : i32 = 189;
pub const KEY_F20 : i32 = 190;
pub const KEY_F21 : i32 = 191;
pub const KEY_F22 : i32 = 192;
pub const KEY_F23 : i32 = 193;
pub const KEY_F24 : i32 = 194;
pub const KEY_PLAYCD : i32 = 200;
pub const KEY_PAUSECD : i32 = 201;
pub const KEY_PROG3 : i32 = 202;
pub const KEY_PROG4 : i32 = 203;
pub const KEY_DASHBOARD : i32 = 204;
pub const KEY_SUSPEND : i32 = 205;
pub const KEY_CLOSE : i32 = 206;
pub const KEY_PLAY : i32 = 207;
pub const KEY_FASTFORWARD : i32 = 208;
pub const KEY_BASSBOOST : i32 = 209;
pub const KEY_PRINT : i32 = 210;
pub const KEY_HP : i32 = 211;
pub const KEY_CAMERA : i32 = 212;
pub const KEY_SOUND : i32 = 213;
pub const KEY_QUESTION : i32 = 214;
pub const KEY_EMAIL : i32 = 215;
pub const KEY_CHAT : i32 = 216;
pub const KEY_SEARCH : i32 = 217;
pub const KEY_CONNECT : i32 = 218;
pub const KEY_FINANCE : i32 = 219;
pub const KEY_SPORT : i32 = 220;
pub const KEY_SHOP : i32 = 221;
pub const KEY_ALTERASE : i32 = 222;
pub const KEY_CANCEL : i32 = 223;
pub const KEY_BRIGHTNESSDOWN : i32 = 224;
pub const KEY_BRIGHTNESSUP : i32 = 225;
pub const KEY_MEDIA : i32 = 226;
pub const KEY_SWITCHVIDEOMODE : i32 = 227;
pub const KEY_KBDILLUMTOGGLE : i32 = 228;
pub const KEY_KBDILLUMDOWN : i32 = 229;
pub const KEY_KBDILLUMUP : i32 = 230;
pub const KEY_SEND : i32 = 231;
pub const KEY_REPLY : i32 = 232;
pub const KEY_FORWARDMAIL : i32 = 233;
pub const KEY_SAVE : i32 = 234;
pub const KEY_DOCUMENTS : i32 = 235;
pub const KEY_BATTERY : i32 = 236;
pub const KEY_BLUETOOTH : i32 = 237;
pub const KEY_WLAN : i32 = 238;
pub const KEY_UWB : i32 = 239;
pub const KEY_UNKNOWN : i32 = 240;
pub const KEY_VIDEO_NEXT : i32 = 241;
pub const KEY_VIDEO_PREV : i32 = 242;
pub const KEY_BRIGHTNESS_CYCLE : i32 = 243;
pub const KEY_BRIGHTNESS_AUTO : i32 = 244;
pub const KEY_BRIGHTNESS_ZERO : i32 = KEY_BRIGHTNESS_AUTO;
pub const KEY_DISPLAY_OFF : i32 = 245;
pub const KEY_WWAN : i32 = 246;
pub const KEY_WIMAX : i32 = KEY_WWAN;
pub const KEY_RFKILL : i32 = 247;
pub const KEY_MICMUTE : i32 = 248;
pub const BTN_MISC : i32 = 0x100;
pub const BTN_0 : i32 = 0x100;
pub const BTN_1 : i32 = 0x101;
pub const BTN_2 : i32 = 0x102;
pub const BTN_3 : i32 = 0x103;
pub const BTN_4 : i32 = 0x104;
pub const BTN_5 : i32 = 0x105;
pub const BTN_6 : i32 = 0x106;
pub const BTN_7 : i32 = 0x107;
pub const BTN_8 : i32 = 0x108;
pub const BTN_9 : i32 = 0x109;
pub const BTN_MOUSE : i32 = 0x110;
pub const BTN_LEFT : i32 = 0x110;
pub const BTN_RIGHT : i32 = 0x111;
pub const BTN_MIDDLE : i32 = 0x112;
pub const BTN_SIDE : i32 = 0x113;
pub const BTN_EXTRA : i32 = 0x114;
pub const BTN_FORWARD : i32 = 0x115;
pub const BTN_BACK : i32 = 0x116;
pub const BTN_TASK : i32 = 0x117;
pub const BTN_JOYSTICK : i32 = 0x120;
pub const BTN_TRIGGER : i32 = 0x120;
pub const BTN_THUMB : i32 = 0x121;
pub const BTN_THUMB2 : i32 = 0x122;
pub const BTN_TOP : i32 = 0x123;
pub const BTN_TOP2 : i32 = 0x124;
pub const BTN_PINKIE : i32 = 0x125;
pub const BTN_BASE : i32 = 0x126;
pub const BTN_BASE2 : i32 = 0x127;
pub const BTN_BASE3 : i32 = 0x128;
pub const BTN_BASE4 : i32 = 0x129;
pub const BTN_BASE5 : i32 = 0x12a;
pub const BTN_BASE6 : i32 = 0x12b;
pub const BTN_DEAD : i32 = 0x12f;
pub const BTN_GAMEPAD : i32 = 0x130;
pub const BTN_SOUTH : i32 = 0x130;
pub const BTN_A : i32 = BTN_SOUTH;
pub const BTN_EAST : i32 = 0x131;
pub const BTN_B : i32 = BTN_EAST;
pub const BTN_C : i32 = 0x132;
pub const BTN_NORTH : i32 = 0x133;
pub const BTN_X : i32 = BTN_NORTH;
pub const BTN_WEST : i32 = 0x134;
pub const BTN_Y : i32 = BTN_WEST;
pub const BTN_Z : i32 = 0x135;
pub const BTN_TL : i32 = 0x136;
pub const BTN_TR : i32 = 0x137;
pub const BTN_TL2 : i32 = 0x138;
pub const BTN_TR2 : i32 = 0x139;
pub const BTN_SELECT : i32 = 0x13a;
pub const BTN_START : i32 = 0x13b;
pub const BTN_MODE : i32 = 0x13c;
pub const BTN_THUMBL : i32 = 0x13d;
pub const BTN_THUMBR : i32 = 0x13e;
pub const BTN_DIGI : i32 = 0x140;
pub const BTN_TOOL_PEN : i32 = 0x140;
pub const BTN_TOOL_RUBBER : i32 = 0x141;
pub const BTN_TOOL_BRUSH : i32 = 0x142;
pub const BTN_TOOL_PENCIL : i32 = 0x143;
pub const BTN_TOOL_AIRBRUSH : i32 = 0x144;
pub const BTN_TOOL_FINGER : i32 = 0x145;
pub const BTN_TOOL_MOUSE : i32 = 0x146;
pub const BTN_TOOL_LENS : i32 = 0x147;
pub const BTN_TOOL_QUINTTAP : i32 = 0x148;
pub const BTN_STYLUS3 : i32 = 0x149;
pub const BTN_TOUCH : i32 = 0x14a;
pub const BTN_STYLUS : i32 = 0x14b;
pub const BTN_STYLUS2 : i32 = 0x14c;
pub const BTN_TOOL_DOUBLETAP : i32 = 0x14d;
pub const BTN_TOOL_TRIPLETAP : i32 = 0x14e;
pub const BTN_TOOL_QUADTAP : i32 = 0x14f;
pub const BTN_WHEEL : i32 = 0x150;
pub const BTN_GEAR_DOWN : i32 = 0x150;
pub const BTN_GEAR_UP : i32 = 0x151;
pub const KEY_OK : i32 = 0x160;
pub const KEY_SELECT : i32 = 0x161;
pub const KEY_GOTO : i32 = 0x162;
pub const KEY_CLEAR : i32 = 0x163;
pub const KEY_POWER2 : i32 = 0x164;
pub const KEY_OPTION : i32 = 0x165;
pub const KEY_INFO : i32 = 0x166;
pub const KEY_TIME : i32 = 0x167;
pub const KEY_VENDOR : i32 = 0x168;
pub const KEY_ARCHIVE : i32 = 0x169;
pub const KEY_PROGRAM : i32 = 0x16a;
pub const KEY_CHANNEL : i32 = 0x16b;
pub const KEY_FAVORITES : i32 = 0x16c;
pub const KEY_EPG : i32 = 0x16d;
pub const KEY_PVR : i32 = 0x16e;
pub const KEY_MHP : i32 = 0x16f;
pub const KEY_LANGUAGE : i32 = 0x170;
pub const KEY_TITLE : i32 = 0x171;
pub const KEY_SUBTITLE : i32 = 0x172;
pub const KEY_ANGLE : i32 = 0x173;
pub const KEY_FULL_SCREEN : i32 = 0x174;
pub const KEY_ZOOM : i32 = KEY_FULL_SCREEN;
pub const KEY_MODE : i32 = 0x175;
pub const KEY_KEYBOARD : i32 = 0x176;
pub const KEY_ASPECT_RATIO : i32 = 0x177;
pub const KEY_SCREEN : i32 = KEY_ASPECT_RATIO;
pub const KEY_PC : i32 = 0x178;
pub const KEY_TV : i32 = 0x179;
pub const KEY_TV2 : i32 = 0x17a;
pub const KEY_VCR : i32 = 0x17b;
pub const KEY_VCR2 : i32 = 0x17c;
pub const KEY_SAT : i32 = 0x17d;
pub const KEY_SAT2 : i32 = 0x17e;
pub const KEY_CD : i32 = 0x17f;
pub const KEY_TAPE : i32 = 0x180;
pub const KEY_RADIO : i32 = 0x181;
pub const KEY_TUNER : i32 = 0x182;
pub const KEY_PLAYER : i32 = 0x183;
pub const KEY_TEXT : i32 = 0x184;
pub const KEY_DVD : i32 = 0x185;
pub const KEY_AUX : i32 = 0x186;
pub const KEY_MP3 : i32 = 0x187;
pub const KEY_AUDIO : i32 = 0x188;
pub const KEY_VIDEO : i32 = 0x189;
pub const KEY_DIRECTORY : i32 = 0x18a;
pub const KEY_LIST : i32 = 0x18b;
pub const KEY_MEMO : i32 = 0x18c;
pub const KEY_CALENDAR : i32 = 0x18d;
pub const KEY_RED : i32 = 0x18e;
pub const KEY_GREEN : i32 = 0x18f;
pub const KEY_YELLOW : i32 = 0x190;
pub const KEY_BLUE : i32 = 0x191;
pub const KEY_CHANNELUP : i32 = 0x192;
pub const KEY_CHANNELDOWN : i32 = 0x193;
pub const KEY_FIRST : i32 = 0x194;
pub const KEY_LAST : i32 = 0x195;
pub const KEY_AB : i32 = 0x196;
pub const KEY_NEXT : i32 = 0x197;
pub const KEY_RESTART : i32 = 0x198;
pub const KEY_SLOW : i32 = 0x199;
pub const KEY_SHUFFLE : i32 = 0x19a;
pub const KEY_BREAK : i32 = 0x19b;
pub const KEY_PREVIOUS : i32 = 0x19c;
pub const KEY_DIGITS : i32 = 0x19d;
pub const KEY_TEEN : i32 = 0x19e;
pub const KEY_TWEN : i32 = 0x19f;
pub const KEY_VIDEOPHONE : i32 = 0x1a0;
pub const KEY_GAMES : i32 = 0x1a1;
pub const KEY_ZOOMIN : i32 = 0x1a2;
pub const KEY_ZOOMOUT : i32 = 0x1a3;
pub const KEY_ZOOMRESET : i32 = 0x1a4;
pub const KEY_WORDPROCESSOR : i32 = 0x1a5;
pub const KEY_EDITOR : i32 = 0x1a6;
pub const KEY_SPREADSHEET : i32 = 0x1a7;
pub const KEY_GRAPHICSEDITOR : i32 = 0x1a8;
pub const KEY_PRESENTATION : i32 = 0x1a9;
pub const KEY_DATABASE : i32 = 0x1aa;
pub const KEY_NEWS : i32 = 0x1ab;
pub const KEY_VOICEMAIL : i32 = 0x1ac;
pub const KEY_ADDRESSBOOK : i32 = 0x1ad;
pub const KEY_MESSENGER : i32 = 0x1ae;
pub const KEY_DISPLAYTOGGLE : i32 = 0x1af;
pub const KEY_BRIGHTNESS_TOGGLE : i32 = KEY_DISPLAYTOGGLE;
pub const KEY_SPELLCHECK : i32 = 0x1b0;
pub const KEY_LOGOFF : i32 = 0x1b1;
pub const KEY_DOLLAR : i32 = 0x1b2;
pub const KEY_EURO : i32 = 0x1b3;
pub const KEY_FRAMEBACK : i32 = 0x1b4;
pub const KEY_FRAMEFORWARD : i32 = 0x1b5;
pub const KEY_CONTEXT_MENU : i32 = 0x1b6;
pub const KEY_MEDIA_REPEAT : i32 = 0x1b7;
pub const KEY_10CHANNELSUP : i32 = 0x1b8;
pub const KEY_10CHANNELSDOWN : i32 = 0x1b9;
pub const KEY_IMAGES : i32 = 0x1ba;
pub const KEY_DEL_EOL : i32 = 0x1c0;
pub const KEY_DEL_EOS : i32 = 0x1c1;
pub const KEY_INS_LINE : i32 = 0x1c2;
pub const KEY_DEL_LINE : i32 = 0x1c3;
pub const KEY_FN : i32 = 0x1d0;
pub const KEY_FN_ESC : i32 = 0x1d1;
pub const KEY_FN_F1 : i32 = 0x1d2;
pub const KEY_FN_F2 : i32 = 0x1d3;
pub const KEY_FN_F3 : i32 = 0x1d4;
pub const KEY_FN_F4 : i32 = 0x1d5;
pub const KEY_FN_F5 : i32 = 0x1d6;
pub const KEY_FN_F6 : i32 = 0x1d7;
pub const KEY_FN_F7 : i32 = 0x1d8;
pub const KEY_FN_F8 : i32 = 0x1d9;
pub const KEY_FN_F9 : i32 = 0x1da;
pub const KEY_FN_F10 : i32 = 0x1db;
pub const KEY_FN_F11 : i32 = 0x1dc;
pub const KEY_FN_F12 : i32 = 0x1dd;
pub const KEY_FN_1 : i32 = 0x1de;
pub const KEY_FN_2 : i32 = 0x1df;
pub const KEY_FN_D : i32 = 0x1e0;
pub const KEY_FN_E : i32 = 0x1e1;
pub const KEY_FN_F : i32 = 0x1e2;
pub const KEY_FN_S : i32 = 0x1e3;
pub const KEY_FN_B : i32 = 0x1e4;
pub const KEY_BRL_DOT1 : i32 = 0x1f1;
pub const KEY_BRL_DOT2 : i32 = 0x1f2;
pub const KEY_BRL_DOT3 : i32 = 0x1f3;
pub const KEY_BRL_DOT4 : i32 = 0x1f4;
pub const KEY_BRL_DOT5 : i32 = 0x1f5;
pub const KEY_BRL_DOT6 : i32 = 0x1f6;
pub const KEY_BRL_DOT7 : i32 = 0x1f7;
pub const KEY_BRL_DOT8 : i32 = 0x1f8;
pub const KEY_BRL_DOT9 : i32 = 0x1f9;
pub const KEY_BRL_DOT10 : i32 = 0x1fa;
pub const KEY_NUMERIC_0 : i32 = 0x200;
pub const KEY_NUMERIC_1 : i32 = 0x201;
pub const KEY_NUMERIC_2 : i32 = 0x202;
pub const KEY_NUMERIC_3 : i32 = 0x203;
pub const KEY_NUMERIC_4 : i32 = 0x204;
pub const KEY_NUMERIC_5 : i32 = 0x205;
pub const KEY_NUMERIC_6 : i32 = 0x206;
pub const KEY_NUMERIC_7 : i32 = 0x207;
pub const KEY_NUMERIC_8 : i32 = 0x208;
pub const KEY_NUMERIC_9 : i32 = 0x209;
pub const KEY_NUMERIC_STAR : i32 = 0x20a;
pub const KEY_NUMERIC_POUND : i32 = 0x20b;
pub const KEY_NUMERIC_A : i32 = 0x20c;
pub const KEY_NUMERIC_B : i32 = 0x20d;
pub const KEY_NUMERIC_C : i32 = 0x20e;
pub const KEY_NUMERIC_D : i32 = 0x20f;
pub const KEY_CAMERA_FOCUS : i32 = 0x210;
pub const KEY_WPS_BUTTON : i32 = 0x211;
pub const KEY_TOUCHPAD_TOGGLE : i32 = 0x212;
pub const KEY_TOUCHPAD_ON : i32 = 0x213;
pub const KEY_TOUCHPAD_OFF : i32 = 0x214;
pub const KEY_CAMERA_ZOOMIN : i32 = 0x215;
pub const KEY_CAMERA_ZOOMOUT : i32 = 0x216;
pub const KEY_CAMERA_UP : i32 = 0x217;
pub const KEY_CAMERA_DOWN : i32 = 0x218;
pub const KEY_CAMERA_LEFT : i32 = 0x219;
pub const KEY_CAMERA_RIGHT : i32 = 0x21a;
pub const KEY_ATTENDANT_ON : i32 = 0x21b;
pub const KEY_ATTENDANT_OFF : i32 = 0x21c;
pub const KEY_ATTENDANT_TOGGLE : i32 = 0x21d;
pub const KEY_LIGHTS_TOGGLE : i32 = 0x21e;
pub const BTN_DPAD_UP : i32 = 0x220;
pub const BTN_DPAD_DOWN : i32 = 0x221;
pub const BTN_DPAD_LEFT : i32 = 0x222;
pub const BTN_DPAD_RIGHT : i32 = 0x223;
pub const KEY_ALS_TOGGLE : i32 = 0x230;
pub const KEY_ROTATE_LOCK_TOGGLE : i32 = 0x231;
pub const KEY_BUTTONCONFIG : i32 = 0x240;
pub const KEY_TASKMANAGER : i32 = 0x241;
pub const KEY_JOURNAL : i32 = 0x242;
pub const KEY_CONTROLPANEL : i32 = 0x243;
pub const KEY_APPSELECT : i32 = 0x244;
pub const KEY_SCREENSAVER : i32 = 0x245;
pub const KEY_VOICECOMMAND : i32 = 0x246;
pub const KEY_ASSISTANT : i32 = 0x247;
pub const KEY_KBD_LAYOUT_NEXT : i32 = 0x248;
pub const KEY_BRIGHTNESS_MIN : i32 = 0x250;
pub const KEY_BRIGHTNESS_MAX : i32 = 0x251;
pub const KEY_KBDINPUTASSIST_PREV : i32 = 0x260;
pub const KEY_KBDINPUTASSIST_NEXT : i32 = 0x261;
pub const KEY_KBDINPUTASSIST_PREVGROUP : i32 = 0x262;
pub const KEY_KBDINPUTASSIST_NEXTGROUP : i32 = 0x263;
pub const KEY_KBDINPUTASSIST_ACCEPT : i32 = 0x264;
pub const KEY_KBDINPUTASSIST_CANCEL : i32 = 0x265;
pub const KEY_RIGHT_UP : i32 = 0x266;
pub const KEY_RIGHT_DOWN : i32 = 0x267;
pub const KEY_LEFT_UP : i32 = 0x268;
pub const KEY_LEFT_DOWN : i32 = 0x269;
pub const KEY_ROOT_MENU : i32 = 0x26a;
pub const KEY_MEDIA_TOP_MENU : i32 = 0x26b;
pub const KEY_NUMERIC_11 : i32 = 0x26c;
pub const KEY_NUMERIC_12 : i32 = 0x26d;
pub const KEY_AUDIO_DESC : i32 = 0x26e;
pub const KEY_3D_MODE : i32 = 0x26f;
pub const KEY_NEXT_FAVORITE : i32 = 0x270;
pub const KEY_STOP_RECORD : i32 = 0x271;
pub const KEY_PAUSE_RECORD : i32 = 0x272;
pub const KEY_VOD : i32 = 0x273;
pub const KEY_UNMUTE : i32 = 0x274;
pub const KEY_FASTREVERSE : i32 = 0x275;
pub const KEY_SLOWREVERSE : i32 = 0x276;
pub const KEY_DATA : i32 = 0x277;
pub const KEY_ONSCREEN_KEYBOARD : i32 = 0x278;
pub const KEY_PRIVACY_SCREEN_TOGGLE : i32 = 0x279;
pub const KEY_SELECTIVE_SCREENSHOT : i32 = 0x27a;
pub const KEY_MACRO1 : i32 = 0x290;
pub const KEY_MACRO2 : i32 = 0x291;
pub const KEY_MACRO3 : i32 = 0x292;
pub const KEY_MACRO4 : i32 = 0x293;
pub const KEY_MACRO5 : i32 = 0x294;
pub const KEY_MACRO6 : i32 = 0x295;
pub const KEY_MACRO7 : i32 = 0x296;
pub const KEY_MACRO8 : i32 = 0x297;
pub const KEY_MACRO9 : i32 = 0x298;
pub const KEY_MACRO10 : i32 = 0x299;
pub const KEY_MACRO11 : i32 = 0x29a;
pub const KEY_MACRO12 : i32 = 0x29b;
pub const KEY_MACRO13 : i32 = 0x29c;
pub const KEY_MACRO14 : i32 = 0x29d;
pub const KEY_MACRO15 : i32 = 0x29e;
pub const KEY_MACRO16 : i32 = 0x29f;
pub const KEY_MACRO17 : i32 = 0x2a0;
pub const KEY_MACRO18 : i32 = 0x2a1;
pub const KEY_MACRO19 : i32 = 0x2a2;
pub const KEY_MACRO20 : i32 = 0x2a3;
pub const KEY_MACRO21 : i32 = 0x2a4;
pub const KEY_MACRO22 : i32 = 0x2a5;
pub const KEY_MACRO23 : i32 = 0x2a6;
pub const KEY_MACRO24 : i32 = 0x2a7;
pub const KEY_MACRO25 : i32 = 0x2a8;
pub const KEY_MACRO26 : i32 = 0x2a9;
pub const KEY_MACRO27 : i32 = 0x2aa;
pub const KEY_MACRO28 : i32 = 0x2ab;
pub const KEY_MACRO29 : i32 = 0x2ac;
pub const KEY_MACRO30 : i32 = 0x2ad;
pub const KEY_MACRO_RECORD_START : i32 = 0x2b0;
pub const KEY_MACRO_RECORD_STOP : i32 = 0x2b1;
pub const KEY_MACRO_PRESET_CYCLE : i32 = 0x2b2;
pub const KEY_MACRO_PRESET1 : i32 = 0x2b3;
pub const KEY_MACRO_PRESET2 : i32 = 0x2b4;
pub const KEY_MACRO_PRESET3 : i32 = 0x2b5;
pub const KEY_KBD_LCD_MENU1 : i32 = 0x2b8;
pub const KEY_KBD_LCD_MENU2 : i32 = 0x2b9;
pub const KEY_KBD_LCD_MENU3 : i32 = 0x2ba;
pub const KEY_KBD_LCD_MENU4 : i32 = 0x2bb;
pub const KEY_KBD_LCD_MENU5 : i32 = 0x2bc;
pub const BTN_TRIGGER_HAPPY : i32 = 0x2c0;
pub const BTN_TRIGGER_HAPPY1 : i32 = 0x2c0;
pub const BTN_TRIGGER_HAPPY2 : i32 = 0x2c1;
pub const BTN_TRIGGER_HAPPY3 : i32 = 0x2c2;
pub const BTN_TRIGGER_HAPPY4 : i32 = 0x2c3;
pub const BTN_TRIGGER_HAPPY5 : i32 = 0x2c4;
pub const BTN_TRIGGER_HAPPY6 : i32 = 0x2c5;
pub const BTN_TRIGGER_HAPPY7 : i32 = 0x2c6;
pub const BTN_TRIGGER_HAPPY8 : i32 = 0x2c7;
pub const BTN_TRIGGER_HAPPY9 : i32 = 0x2c8;
pub const BTN_TRIGGER_HAPPY10 : i32 = 0x2c9;
pub const BTN_TRIGGER_HAPPY11 : i32 = 0x2ca;
pub const BTN_TRIGGER_HAPPY12 : i32 = 0x2cb;
pub const BTN_TRIGGER_HAPPY13 : i32 = 0x2cc;
pub const BTN_TRIGGER_HAPPY14 : i32 = 0x2cd;
pub const BTN_TRIGGER_HAPPY15 : i32 = 0x2ce;
pub const BTN_TRIGGER_HAPPY16 : i32 = 0x2cf;
pub const BTN_TRIGGER_HAPPY17 : i32 = 0x2d0;
pub const BTN_TRIGGER_HAPPY18 : i32 = 0x2d1;
pub const BTN_TRIGGER_HAPPY19 : i32 = 0x2d2;
pub const BTN_TRIGGER_HAPPY20 : i32 = 0x2d3;
pub const BTN_TRIGGER_HAPPY21 : i32 = 0x2d4;
pub const BTN_TRIGGER_HAPPY22 : i32 = 0x2d5;
pub const BTN_TRIGGER_HAPPY23 : i32 = 0x2d6;
pub const BTN_TRIGGER_HAPPY24 : i32 = 0x2d7;
pub const BTN_TRIGGER_HAPPY25 : i32 = 0x2d8;
pub const BTN_TRIGGER_HAPPY26 : i32 = 0x2d9;
pub const BTN_TRIGGER_HAPPY27 : i32 = 0x2da;
pub const BTN_TRIGGER_HAPPY28 : i32 = 0x2db;
pub const BTN_TRIGGER_HAPPY29 : i32 = 0x2dc;
pub const BTN_TRIGGER_HAPPY30 : i32 = 0x2dd;
pub const BTN_TRIGGER_HAPPY31 : i32 = 0x2de;
pub const BTN_TRIGGER_HAPPY32 : i32 = 0x2df;
pub const BTN_TRIGGER_HAPPY33 : i32 = 0x2e0;
pub const BTN_TRIGGER_HAPPY34 : i32 = 0x2e1;
pub const BTN_TRIGGER_HAPPY35 : i32 = 0x2e2;
pub const BTN_TRIGGER_HAPPY36 : i32 = 0x2e3;
pub const BTN_TRIGGER_HAPPY37 : i32 = 0x2e4;
pub const BTN_TRIGGER_HAPPY38 : i32 = 0x2e5;
pub const BTN_TRIGGER_HAPPY39 : i32 = 0x2e6;
pub const BTN_TRIGGER_HAPPY40 : i32 = 0x2e7;
pub const KEY_MIN_INTERESTING : i32 = KEY_MUTE;
pub const KEY_MAX : i32 = 0x2ff;
pub const KEY_CNT : i32 = KEY_MAX+1;
pub const REL_X : i32 = 0x00;
pub const REL_Y : i32 = 0x01;
pub const REL_Z : i32 = 0x02;
pub const REL_RX : i32 = 0x03;
pub const REL_RY : i32 = 0x04;
pub const REL_RZ : i32 = 0x05;
pub const REL_HWHEEL : i32 = 0x06;
pub const REL_DIAL : i32 = 0x07;
pub const REL_WHEEL : i32 = 0x08;
pub const REL_MISC : i32 = 0x09;
pub const REL_RESERVED : i32 = 0x0a;
pub const REL_WHEEL_HI_RES : i32 = 0x0b;
pub const REL_HWHEEL_HI_RES : i32 = 0x0c;
pub const REL_MAX : i32 = 0x0f;
pub const REL_CNT : i32 = REL_MAX+1;
pub const ABS_X : i32 = 0x00;
pub const ABS_Y : i32 = 0x01;
pub const ABS_Z : i32 = 0x02;
pub const ABS_RX : i32 = 0x03;
pub const ABS_RY : i32 = 0x04;
pub const ABS_RZ : i32 = 0x05;
pub const ABS_THROTTLE : i32 = 0x06;
pub const ABS_RUDDER : i32 = 0x07;
pub const ABS_WHEEL : i32 = 0x08;
pub const ABS_GAS : i32 = 0x09;
pub const ABS_BRAKE : i32 = 0x0a;
pub const ABS_HAT0X : i32 = 0x10;
pub const ABS_HAT0Y : i32 = 0x11;
pub const ABS_HAT1X : i32 = 0x12;
pub const ABS_HAT1Y : i32 = 0x13;
pub const ABS_HAT2X : i32 = 0x14;
pub const ABS_HAT2Y : i32 = 0x15;
pub const ABS_HAT3X : i32 = 0x16;
pub const ABS_HAT3Y : i32 = 0x17;
pub const ABS_PRESSURE : i32 = 0x18;
pub const ABS_DISTANCE : i32 = 0x19;
pub const ABS_TILT_X : i32 = 0x1a;
pub const ABS_TILT_Y : i32 = 0x1b;
pub const ABS_TOOL_WIDTH : i32 = 0x1c;
pub const ABS_VOLUME : i32 = 0x20;
pub const ABS_MISC : i32 = 0x28;
pub const ABS_RESERVED : i32 = 0x2e;
pub const ABS_MT_SLOT : i32 = 0x2f;
pub const ABS_MT_TOUCH_MAJOR : i32 = 0x30;
pub const ABS_MT_TOUCH_MINOR : i32 = 0x31;
pub const ABS_MT_WIDTH_MAJOR : i32 = 0x32;
pub const ABS_MT_WIDTH_MINOR : i32 = 0x33;
pub const ABS_MT_ORIENTATION : i32 = 0x34;
pub const ABS_MT_POSITION_X : i32 = 0x35;
pub const ABS_MT_POSITION_Y : i32 = 0x36;
pub const ABS_MT_TOOL_TYPE : i32 = 0x37;
pub const ABS_MT_BLOB_ID : i32 = 0x38;
pub const ABS_MT_TRACKING_ID : i32 = 0x39;
pub const ABS_MT_PRESSURE : i32 = 0x3a;
pub const ABS_MT_DISTANCE : i32 = 0x3b;
pub const ABS_MT_TOOL_X : i32 = 0x3c;
pub const ABS_MT_TOOL_Y : i32 = 0x3d;
pub const ABS_MAX : i32 = 0x3f;
pub const ABS_CNT : i32 = ABS_MAX+1;
pub const SW_LID : i32 = 0x00;
pub const SW_TABLET_MODE : i32 = 0x01;
pub const SW_HEADPHONE_INSERT : i32 = 0x02;
pub const SW_RFKILL_ALL : i32 = 0x03;
pub const SW_RADIO : i32 = SW_RFKILL_ALL;
pub const SW_MICROPHONE_INSERT : i32 = 0x04;
pub const SW_DOCK : i32 = 0x05;
pub const SW_LINEOUT_INSERT : i32 = 0x06;
pub const SW_JACK_PHYSICAL_INSERT : i32 = 0x07;
pub const SW_VIDEOOUT_INSERT : i32 = 0x08;
pub const SW_CAMERA_LENS_COVER : i32 = 0x09;
pub const SW_KEYPAD_SLIDE : i32 = 0x0a;
pub const SW_FRONT_PROXIMITY : i32 = 0x0b;
pub const SW_ROTATE_LOCK : i32 = 0x0c;
pub const SW_LINEIN_INSERT : i32 = 0x0d;
pub const SW_MUTE_DEVICE : i32 = 0x0e;
pub const SW_PEN_INSERTED : i32 = 0x0f;
pub const SW_MACHINE_COVER : i32 = 0x10;
pub const SW_MAX : i32 = 0x10;
pub const SW_CNT : i32 = SW_MAX+1;
pub const MSC_SERIAL : i32 = 0x00;
pub const MSC_PULSELED : i32 = 0x01;
pub const MSC_GESTURE : i32 = 0x02;
pub const MSC_RAW : i32 = 0x03;
pub const MSC_SCAN : i32 = 0x04;
pub const MSC_TIMESTAMP : i32 = 0x05;
pub const MSC_MAX : i32 = 0x07;
pub const MSC_CNT : i32 = MSC_MAX+1;
pub const LED_NUML : i32 = 0x00;
pub const LED_CAPSL : i32 = 0x01;
pub const LED_SCROLLL : i32 = 0x02;
pub const LED_COMPOSE : i32 = 0x03;
pub const LED_KANA : i32 = 0x04;
pub const LED_SLEEP : i32 = 0x05;
pub const LED_SUSPEND : i32 = 0x06;
pub const LED_MUTE : i32 = 0x07;
pub const LED_MISC : i32 = 0x08;
pub const LED_MAIL : i32 = 0x09;
pub const LED_CHARGING : i32 = 0x0a;
pub const LED_MAX : i32 = 0x0f;
pub const LED_CNT : i32 = LED_MAX+1;
pub const REP_DELAY : i32 = 0x00;
pub const REP_PERIOD : i32 = 0x01;
pub const REP_MAX : i32 = 0x01;
pub const REP_CNT : i32 = REP_MAX+1;
pub const SND_CLICK : i32 = 0x00;
pub const SND_BELL : i32 = 0x01;
pub const SND_TONE : i32 = 0x02;
pub const SND_MAX : i32 = 0x07;
pub const SND_CNT : i32 = SND_MAX+1;

pub static ALL_KEYS: &'static [i32] = &[
        KEY_RESERVED,
        KEY_ESC,
        KEY_1,
        KEY_2,
        KEY_3,
        KEY_4,
        KEY_5,
        KEY_6,
        KEY_7,
        KEY_8,
        KEY_9,
        KEY_0,
        KEY_MINUS,
        KEY_EQUAL,
        KEY_BACKSPACE,
        KEY_TAB,
        KEY_Q,
        KEY_W,
        KEY_E,
        KEY_R,
        KEY_T,
        KEY_Y,
        KEY_U,
        KEY_I,
        KEY_O,
        KEY_P,
        KEY_LEFTBRACE,
        KEY_RIGHTBRACE,
        KEY_ENTER,
        KEY_LEFTCTRL,
        KEY_A,
        KEY_S,
        KEY_D,
        KEY_F,
        KEY_G,
        KEY_H,
        KEY_J,
        KEY_K,
        KEY_L,
        KEY_SEMICOLON,
        KEY_APOSTROPHE,
        KEY_GRAVE,
        KEY_LEFTSHIFT,
        KEY_BACKSLASH,
        KEY_Z,
        KEY_X,
        KEY_C,
        KEY_V,
        KEY_B,
        KEY_N,
        KEY_M,
        KEY_COMMA,
        KEY_DOT,
        KEY_SLASH,
        KEY_RIGHTSHIFT,
        KEY_KPASTERISK,
        KEY_LEFTALT,
        KEY_SPACE,
        KEY_CAPSLOCK,
        KEY_F1,
        KEY_F2,
        KEY_F3,
        KEY_F4,
        KEY_F5,
        KEY_F6,
        KEY_F7,
        KEY_F8,
        KEY_F9,
        KEY_F10,
        KEY_NUMLOCK,
        KEY_SCROLLLOCK,
        KEY_KP7,
        KEY_KP8,
        KEY_KP9,
        KEY_KPMINUS,
        KEY_KP4,
        KEY_KP5,
        KEY_KP6,
        KEY_KPPLUS,
        KEY_KP1,
        KEY_KP2,
        KEY_KP3,
        KEY_KP0,
        KEY_KPDOT,
        KEY_ZENKAKUHANKAKU,
        KEY_102ND,
        KEY_F11,
        KEY_F12,
        KEY_RO,
        KEY_KATAKANA,
        KEY_HIRAGANA,
        KEY_HENKAN,
        KEY_KATAKANAHIRAGANA,
        KEY_MUHENKAN,
        KEY_KPJPCOMMA,
        KEY_KPENTER,
        KEY_RIGHTCTRL,
        KEY_KPSLASH,
        KEY_SYSRQ,
        KEY_RIGHTALT,
        KEY_LINEFEED,
        KEY_HOME,
        KEY_UP,
        KEY_PAGEUP,
        KEY_LEFT,
        KEY_RIGHT,
        KEY_END,
        KEY_DOWN,
        KEY_PAGEDOWN,
        KEY_INSERT,
        KEY_DELETE,
        KEY_MACRO,
        KEY_MUTE,
        KEY_VOLUMEDOWN,
        KEY_VOLUMEUP,
        KEY_POWER,
        KEY_KPEQUAL,
        KEY_KPPLUSMINUS,
        KEY_PAUSE,
        KEY_SCALE,
        KEY_KPCOMMA,
        KEY_HANGEUL,
        KEY_HANGUEL,
        KEY_HANJA,
        KEY_YEN,
        KEY_LEFTMETA,
        KEY_RIGHTMETA,
        KEY_COMPOSE,
        KEY_STOP,
        KEY_AGAIN,
        KEY_PROPS,
        KEY_UNDO,
        KEY_FRONT,
        KEY_COPY,
        KEY_OPEN,
        KEY_PASTE,
        KEY_FIND,
        KEY_CUT,
        KEY_HELP,
        KEY_MENU,
        KEY_CALC,
        KEY_SETUP,
        KEY_SLEEP,
        KEY_WAKEUP,
        KEY_FILE,
        KEY_SENDFILE,
        KEY_DELETEFILE,
        KEY_XFER,
        KEY_PROG1,
        KEY_PROG2,
        KEY_WWW,
        KEY_MSDOS,
        KEY_COFFEE,
        KEY_SCREENLOCK,
        KEY_ROTATE_DISPLAY,
        KEY_DIRECTION,
        KEY_CYCLEWINDOWS,
        KEY_MAIL,
        KEY_BOOKMARKS,
        KEY_COMPUTER,
        KEY_BACK,
        KEY_FORWARD,
        KEY_CLOSECD,
        KEY_EJECTCD,
        KEY_EJECTCLOSECD,
        KEY_NEXTSONG,
        KEY_PLAYPAUSE,
        KEY_PREVIOUSSONG,
        KEY_STOPCD,
        KEY_RECORD,
        KEY_REWIND,
        KEY_PHONE,
        KEY_ISO,
        KEY_CONFIG,
        KEY_HOMEPAGE,
        KEY_REFRESH,
        KEY_EXIT,
        KEY_MOVE,
        KEY_EDIT,
        KEY_SCROLLUP,
        KEY_SCROLLDOWN,
        KEY_KPLEFTPAREN,
        KEY_KPRIGHTPAREN,
        KEY_NEW,
        KEY_REDO,
        KEY_F13,
        KEY_F14,
        KEY_F15,
        KEY_F16,
        KEY_F17,
        KEY_F18,
        KEY_F19,
        KEY_F20,
        KEY_F21,
        KEY_F22,
        KEY_F23,
        KEY_F24,
        KEY_PLAYCD,
        KEY_PAUSECD,
        KEY_PROG3,
        KEY_PROG4,
        KEY_DASHBOARD,
        KEY_SUSPEND,
        KEY_CLOSE,
        KEY_PLAY,
        KEY_FASTFORWARD,
        KEY_BASSBOOST,
        KEY_PRINT,
        KEY_HP,
        KEY_CAMERA,
        KEY_SOUND,
        KEY_QUESTION,
        KEY_EMAIL,
        KEY_CHAT,
        KEY_SEARCH,
        KEY_CONNECT,
        KEY_FINANCE,
        KEY_SPORT,
        KEY_SHOP,
        KEY_ALTERASE,
        KEY_CANCEL,
        KEY_BRIGHTNESSDOWN,
        KEY_BRIGHTNESSUP,
        KEY_MEDIA,
        KEY_SWITCHVIDEOMODE,
        KEY_KBDILLUMTOGGLE,
        KEY_KBDILLUMDOWN,
        KEY_KBDILLUMUP,
        KEY_SEND,
        KEY_REPLY,
        KEY_FORWARDMAIL,
        KEY_SAVE,
        KEY_DOCUMENTS,
        KEY_BATTERY,
        KEY_BLUETOOTH,
        KEY_WLAN,
        KEY_UWB,
        KEY_UNKNOWN,
        KEY_VIDEO_NEXT,
        KEY_VIDEO_PREV,
        KEY_BRIGHTNESS_CYCLE,
        KEY_BRIGHTNESS_AUTO,
        KEY_BRIGHTNESS_ZERO,
        KEY_DISPLAY_OFF,
        KEY_WWAN,
        KEY_WIMAX,
        KEY_RFKILL,
        KEY_MICMUTE,
        BTN_MISC,
        BTN_0,
        BTN_1,
        BTN_2,
        BTN_3,
        BTN_4,
        BTN_5,
        BTN_6,
        BTN_7,
        BTN_8,
        BTN_9,
        BTN_MOUSE,
        BTN_LEFT,
        BTN_RIGHT,
        BTN_MIDDLE,
        BTN_SIDE,
        BTN_EXTRA,
        BTN_FORWARD,
        BTN_BACK,
        BTN_TASK,
        BTN_JOYSTICK,
        BTN_TRIGGER,
        BTN_THUMB,
        BTN_THUMB2,
        BTN_TOP,
        BTN_TOP2,
        BTN_PINKIE,
        BTN_BASE,
        BTN_BASE2,
        BTN_BASE3,
        BTN_BASE4,
        BTN_BASE5,
        BTN_BASE6,
        BTN_DEAD,
        BTN_GAMEPAD,
        BTN_SOUTH,
        BTN_A,
        BTN_EAST,
        BTN_B,
        BTN_C,
        BTN_NORTH,
        BTN_X,
        BTN_WEST,
        BTN_Y,
        BTN_Z,
        BTN_TL,
        BTN_TR,
        BTN_TL2,
        BTN_TR2,
        BTN_SELECT,
        BTN_START,
        BTN_MODE,
        BTN_THUMBL,
        BTN_THUMBR,
        BTN_DIGI,
        BTN_TOOL_PEN,
        BTN_TOOL_RUBBER,
        BTN_TOOL_BRUSH,
        BTN_TOOL_PENCIL,
        BTN_TOOL_AIRBRUSH,
        BTN_TOOL_FINGER,
        BTN_TOOL_MOUSE,
        BTN_TOOL_LENS,
        BTN_TOOL_QUINTTAP,
        BTN_STYLUS3,
        BTN_TOUCH,
        BTN_STYLUS,
        BTN_STYLUS2,
        BTN_TOOL_DOUBLETAP,
        BTN_TOOL_TRIPLETAP,
        BTN_TOOL_QUADTAP,
        BTN_WHEEL,
        BTN_GEAR_DOWN,
        BTN_GEAR_UP,
        KEY_OK,
        KEY_SELECT,
        KEY_GOTO,
        KEY_CLEAR,
        KEY_POWER2,
        KEY_OPTION,
        KEY_INFO,
        KEY_TIME,
        KEY_VENDOR,
        KEY_ARCHIVE,
        KEY_PROGRAM,
        KEY_CHANNEL,
        KEY_FAVORITES,
        KEY_EPG,
        KEY_PVR,
        KEY_MHP,
        KEY_LANGUAGE,
        KEY_TITLE,
        KEY_SUBTITLE,
        KEY_ANGLE,
        KEY_FULL_SCREEN,
        KEY_ZOOM,
        KEY_MODE,
        KEY_KEYBOARD,
        KEY_ASPECT_RATIO,
        KEY_SCREEN,
        KEY_PC,
        KEY_TV,
        KEY_TV2,
        KEY_VCR,
        KEY_VCR2,
        KEY_SAT,
        KEY_SAT2,
        KEY_CD,
        KEY_TAPE,
        KEY_RADIO,
        KEY_TUNER,
        KEY_PLAYER,
        KEY_TEXT,
        KEY_DVD,
        KEY_AUX,
        KEY_MP3,
        KEY_AUDIO,
        KEY_VIDEO,
        KEY_DIRECTORY,
        KEY_LIST,
        KEY_MEMO,
        KEY_CALENDAR,
        KEY_RED,
        KEY_GREEN,
        KEY_YELLOW,
        KEY_BLUE,
        KEY_CHANNELUP,
        KEY_CHANNELDOWN,
        KEY_FIRST,
        KEY_LAST,
        KEY_AB,
        KEY_NEXT,
        KEY_RESTART,
        KEY_SLOW,
        KEY_SHUFFLE,
        KEY_BREAK,
        KEY_PREVIOUS,
        KEY_DIGITS,
        KEY_TEEN,
        KEY_TWEN,
        KEY_VIDEOPHONE,
        KEY_GAMES,
        KEY_ZOOMIN,
        KEY_ZOOMOUT,
        KEY_ZOOMRESET,
        KEY_WORDPROCESSOR,
        KEY_EDITOR,
        KEY_SPREADSHEET,
        KEY_GRAPHICSEDITOR,
        KEY_PRESENTATION,
        KEY_DATABASE,
        KEY_NEWS,
        KEY_VOICEMAIL,
        KEY_ADDRESSBOOK,
        KEY_MESSENGER,
        KEY_DISPLAYTOGGLE,
        KEY_BRIGHTNESS_TOGGLE,
        KEY_SPELLCHECK,
        KEY_LOGOFF,
        KEY_DOLLAR,
        KEY_EURO,
        KEY_FRAMEBACK,
        KEY_FRAMEFORWARD,
        KEY_CONTEXT_MENU,
        KEY_MEDIA_REPEAT,
        KEY_10CHANNELSUP,
        KEY_10CHANNELSDOWN,
        KEY_IMAGES,
        KEY_DEL_EOL,
        KEY_DEL_EOS,
        KEY_INS_LINE,
        KEY_DEL_LINE,
        KEY_FN,
        KEY_FN_ESC,
        KEY_FN_F1,
        KEY_FN_F2,
        KEY_FN_F3,
        KEY_FN_F4,
        KEY_FN_F5,
        KEY_FN_F6,
        KEY_FN_F7,
        KEY_FN_F8,
        KEY_FN_F9,
        KEY_FN_F10,
        KEY_FN_F11,
        KEY_FN_F12,
        KEY_FN_1,
        KEY_FN_2,
        KEY_FN_D,
        KEY_FN_E,
        KEY_FN_F,
        KEY_FN_S,
        KEY_FN_B,
        KEY_BRL_DOT1,
        KEY_BRL_DOT2,
        KEY_BRL_DOT3,
        KEY_BRL_DOT4,
        KEY_BRL_DOT5,
        KEY_BRL_DOT6,
        KEY_BRL_DOT7,
        KEY_BRL_DOT8,
        KEY_BRL_DOT9,
        KEY_BRL_DOT10,
        KEY_NUMERIC_0,
        KEY_NUMERIC_1,
        KEY_NUMERIC_2,
        KEY_NUMERIC_3,
        KEY_NUMERIC_4,
        KEY_NUMERIC_5,
        KEY_NUMERIC_6,
        KEY_NUMERIC_7,
        KEY_NUMERIC_8,
        KEY_NUMERIC_9,
        KEY_NUMERIC_STAR,
        KEY_NUMERIC_POUND,
        KEY_NUMERIC_A,
        KEY_NUMERIC_B,
        KEY_NUMERIC_C,
        KEY_NUMERIC_D,
        KEY_CAMERA_FOCUS,
        KEY_WPS_BUTTON,
        KEY_TOUCHPAD_TOGGLE,
        KEY_TOUCHPAD_ON,
        KEY_TOUCHPAD_OFF,
        KEY_CAMERA_ZOOMIN,
        KEY_CAMERA_ZOOMOUT,
        KEY_CAMERA_UP,
        KEY_CAMERA_DOWN,
        KEY_CAMERA_LEFT,
        KEY_CAMERA_RIGHT,
        KEY_ATTENDANT_ON,
        KEY_ATTENDANT_OFF,
        KEY_ATTENDANT_TOGGLE,
        KEY_LIGHTS_TOGGLE,
        BTN_DPAD_UP,
        BTN_DPAD_DOWN,
        BTN_DPAD_LEFT,
        BTN_DPAD_RIGHT,
        KEY_ALS_TOGGLE,
        KEY_ROTATE_LOCK_TOGGLE,
        KEY_BUTTONCONFIG,
        KEY_TASKMANAGER,
        KEY_JOURNAL,
        KEY_CONTROLPANEL,
        KEY_APPSELECT,
        KEY_SCREENSAVER,
        KEY_VOICECOMMAND,
        KEY_ASSISTANT,
        KEY_KBD_LAYOUT_NEXT,
        KEY_BRIGHTNESS_MIN,
        KEY_KBDINPUTASSIST_PREV,
        KEY_KBDINPUTASSIST_NEXT,
        KEY_KBDINPUTASSIST_PREVGROUP,
        KEY_KBDINPUTASSIST_NEXTGROUP,
        KEY_KBDINPUTASSIST_ACCEPT,
        KEY_KBDINPUTASSIST_CANCEL,
        KEY_RIGHT_UP,
        KEY_RIGHT_DOWN,
        KEY_LEFT_UP,
        KEY_LEFT_DOWN,
        KEY_ROOT_MENU,
        KEY_MEDIA_TOP_MENU,
        KEY_NUMERIC_11,
        KEY_NUMERIC_12,
        KEY_AUDIO_DESC,
        KEY_3D_MODE,
        KEY_NEXT_FAVORITE,
        KEY_STOP_RECORD,
        KEY_PAUSE_RECORD,
        KEY_VOD,
        KEY_UNMUTE,
        KEY_FASTREVERSE,
        KEY_SLOWREVERSE,
        KEY_DATA,
        KEY_ONSCREEN_KEYBOARD,
        KEY_PRIVACY_SCREEN_TOGGLE,
        KEY_SELECTIVE_SCREENSHOT,
        KEY_MACRO1,
        KEY_MACRO2,
        KEY_MACRO3,
        KEY_MACRO4,
        KEY_MACRO5,
        KEY_MACRO6,
        KEY_MACRO7,
        KEY_MACRO8,
        KEY_MACRO9,
        KEY_MACRO10,
        KEY_MACRO11,
        KEY_MACRO12,
        KEY_MACRO13,
        KEY_MACRO14,
        KEY_MACRO15,
        KEY_MACRO16,
        KEY_MACRO17,
        KEY_MACRO18,
        KEY_MACRO19,
        KEY_MACRO20,
        KEY_MACRO21,
        KEY_MACRO22,
        KEY_MACRO23,
        KEY_MACRO24,
        KEY_MACRO25,
        KEY_MACRO26,
        KEY_MACRO27,
        KEY_MACRO28,
        KEY_MACRO29,
        KEY_MACRO30,
        KEY_MACRO_RECORD_START,
        KEY_MACRO_RECORD_STOP,
        KEY_MACRO_PRESET_CYCLE,
        KEY_MACRO_PRESET1,
        KEY_MACRO_PRESET2,
        KEY_MACRO_PRESET3,
        KEY_KBD_LCD_MENU1,
        KEY_KBD_LCD_MENU2,
        KEY_KBD_LCD_MENU3,
        KEY_KBD_LCD_MENU4,
        KEY_KBD_LCD_MENU5,
        BTN_TRIGGER_HAPPY,
        BTN_TRIGGER_HAPPY1,
        BTN_TRIGGER_HAPPY2,
        BTN_TRIGGER_HAPPY3,
        BTN_TRIGGER_HAPPY4,
        BTN_TRIGGER_HAPPY5,
        BTN_TRIGGER_HAPPY6,
        BTN_TRIGGER_HAPPY7,
        BTN_TRIGGER_HAPPY8,
        BTN_TRIGGER_HAPPY9,
        BTN_TRIGGER_HAPPY10,
        BTN_TRIGGER_HAPPY11,
        BTN_TRIGGER_HAPPY12,
        BTN_TRIGGER_HAPPY13,
        BTN_TRIGGER_HAPPY14,
        BTN_TRIGGER_HAPPY15,
        BTN_TRIGGER_HAPPY16,
        BTN_TRIGGER_HAPPY17,
        BTN_TRIGGER_HAPPY18,
        BTN_TRIGGER_HAPPY19,
        BTN_TRIGGER_HAPPY20,
        BTN_TRIGGER_HAPPY21,
        BTN_TRIGGER_HAPPY22,
        BTN_TRIGGER_HAPPY23,
        BTN_TRIGGER_HAPPY24,
        BTN_TRIGGER_HAPPY25,
        BTN_TRIGGER_HAPPY26,
        BTN_TRIGGER_HAPPY27,
        BTN_TRIGGER_HAPPY28,
        BTN_TRIGGER_HAPPY29,
        BTN_TRIGGER_HAPPY30,
        BTN_TRIGGER_HAPPY31,
        BTN_TRIGGER_HAPPY32,
        BTN_TRIGGER_HAPPY33,
        BTN_TRIGGER_HAPPY34,
        BTN_TRIGGER_HAPPY35,
        BTN_TRIGGER_HAPPY36,
        BTN_TRIGGER_HAPPY37,
        BTN_TRIGGER_HAPPY38,
        BTN_TRIGGER_HAPPY39,
        BTN_TRIGGER_HAPPY40,
        KEY_MIN_INTERESTING,
        ];

pub fn get_type_name(type_: i32) -> &'static str {
	match type_ {
		EV_SYN => "EV_SYN",
		EV_KEY => "EV_KEY",
		EV_REL => "EV_REL",
		EV_ABS => "EV_ABS",
		EV_MSC => "EV_MSC",
		EV_SW => "EV_SW",
		EV_LED => "EV_LED",
		EV_SND => "EV_SND",
		EV_REP => "EV_REP",
		_ => "",
	}
}

#[allow(unreachable_patterns)] // Needed because of aliases.
pub fn get_code_name(type_: i32, code: i32) -> &'static str {
	match type_ {
		EV_SYN => match code {
			SYN_REPORT => "SYN_REPORT",
			SYN_CONFIG => "SYN_CONFIG",
			SYN_MT_REPORT => "SYN_MT_REPORT",
			SYN_DROPPED => "SYN_DROPPED",
			_ => "",
		}
		EV_KEY => match code {
			KEY_RESERVED => "KEY_RESERVED",
			KEY_ESC => "KEY_ESC",
			KEY_1 => "KEY_1",
			KEY_2 => "KEY_2",
			KEY_3 => "KEY_3",
			KEY_4 => "KEY_4",
			KEY_5 => "KEY_5",
			KEY_6 => "KEY_6",
			KEY_7 => "KEY_7",
			KEY_8 => "KEY_8",
			KEY_9 => "KEY_9",
			KEY_0 => "KEY_0",
			KEY_MINUS => "KEY_MINUS",
			KEY_EQUAL => "KEY_EQUAL",
			KEY_BACKSPACE => "KEY_BACKSPACE",
			KEY_TAB => "KEY_TAB",
			KEY_Q => "KEY_Q",
			KEY_W => "KEY_W",
			KEY_E => "KEY_E",
			KEY_R => "KEY_R",
			KEY_T => "KEY_T",
			KEY_Y => "KEY_Y",
			KEY_U => "KEY_U",
			KEY_I => "KEY_I",
			KEY_O => "KEY_O",
			KEY_P => "KEY_P",
			KEY_LEFTBRACE => "KEY_LEFTBRACE",
			KEY_RIGHTBRACE => "KEY_RIGHTBRACE",
			KEY_ENTER => "KEY_ENTER",
			KEY_LEFTCTRL => "KEY_LEFTCTRL",
			KEY_A => "KEY_A",
			KEY_S => "KEY_S",
			KEY_D => "KEY_D",
			KEY_F => "KEY_F",
			KEY_G => "KEY_G",
			KEY_H => "KEY_H",
			KEY_J => "KEY_J",
			KEY_K => "KEY_K",
			KEY_L => "KEY_L",
			KEY_SEMICOLON => "KEY_SEMICOLON",
			KEY_APOSTROPHE => "KEY_APOSTROPHE",
			KEY_GRAVE => "KEY_GRAVE",
			KEY_LEFTSHIFT => "KEY_LEFTSHIFT",
			KEY_BACKSLASH => "KEY_BACKSLASH",
			KEY_Z => "KEY_Z",
			KEY_X => "KEY_X",
			KEY_C => "KEY_C",
			KEY_V => "KEY_V",
			KEY_B => "KEY_B",
			KEY_N => "KEY_N",
			KEY_M => "KEY_M",
			KEY_COMMA => "KEY_COMMA",
			KEY_DOT => "KEY_DOT",
			KEY_SLASH => "KEY_SLASH",
			KEY_RIGHTSHIFT => "KEY_RIGHTSHIFT",
			KEY_KPASTERISK => "KEY_KPASTERISK",
			KEY_LEFTALT => "KEY_LEFTALT",
			KEY_SPACE => "KEY_SPACE",
			KEY_CAPSLOCK => "KEY_CAPSLOCK",
			KEY_F1 => "KEY_F1",
			KEY_F2 => "KEY_F2",
			KEY_F3 => "KEY_F3",
			KEY_F4 => "KEY_F4",
			KEY_F5 => "KEY_F5",
			KEY_F6 => "KEY_F6",
			KEY_F7 => "KEY_F7",
			KEY_F8 => "KEY_F8",
			KEY_F9 => "KEY_F9",
			KEY_F10 => "KEY_F10",
			KEY_NUMLOCK => "KEY_NUMLOCK",
			KEY_SCROLLLOCK => "KEY_SCROLLLOCK",
			KEY_KP7 => "KEY_KP7",
			KEY_KP8 => "KEY_KP8",
			KEY_KP9 => "KEY_KP9",
			KEY_KPMINUS => "KEY_KPMINUS",
			KEY_KP4 => "KEY_KP4",
			KEY_KP5 => "KEY_KP5",
			KEY_KP6 => "KEY_KP6",
			KEY_KPPLUS => "KEY_KPPLUS",
			KEY_KP1 => "KEY_KP1",
			KEY_KP2 => "KEY_KP2",
			KEY_KP3 => "KEY_KP3",
			KEY_KP0 => "KEY_KP0",
			KEY_KPDOT => "KEY_KPDOT",
			KEY_ZENKAKUHANKAKU => "KEY_ZENKAKUHANKAKU",
			KEY_102ND => "KEY_102ND",
			KEY_F11 => "KEY_F11",
			KEY_F12 => "KEY_F12",
			KEY_RO => "KEY_RO",
			KEY_KATAKANA => "KEY_KATAKANA",
			KEY_HIRAGANA => "KEY_HIRAGANA",
			KEY_HENKAN => "KEY_HENKAN",
			KEY_KATAKANAHIRAGANA => "KEY_KATAKANAHIRAGANA",
			KEY_MUHENKAN => "KEY_MUHENKAN",
			KEY_KPJPCOMMA => "KEY_KPJPCOMMA",
			KEY_KPENTER => "KEY_KPENTER",
			KEY_RIGHTCTRL => "KEY_RIGHTCTRL",
			KEY_KPSLASH => "KEY_KPSLASH",
			KEY_SYSRQ => "KEY_SYSRQ",
			KEY_RIGHTALT => "KEY_RIGHTALT",
			KEY_LINEFEED => "KEY_LINEFEED",
			KEY_HOME => "KEY_HOME",
			KEY_UP => "KEY_UP",
			KEY_PAGEUP => "KEY_PAGEUP",
			KEY_LEFT => "KEY_LEFT",
			KEY_RIGHT => "KEY_RIGHT",
			KEY_END => "KEY_END",
			KEY_DOWN => "KEY_DOWN",
			KEY_PAGEDOWN => "KEY_PAGEDOWN",
			KEY_INSERT => "KEY_INSERT",
			KEY_DELETE => "KEY_DELETE",
			KEY_MACRO => "KEY_MACRO",
			KEY_MUTE => "KEY_MUTE",
			KEY_VOLUMEDOWN => "KEY_VOLUMEDOWN",
			KEY_VOLUMEUP => "KEY_VOLUMEUP",
			KEY_POWER => "KEY_POWER",
			KEY_KPEQUAL => "KEY_KPEQUAL",
			KEY_KPPLUSMINUS => "KEY_KPPLUSMINUS",
			KEY_PAUSE => "KEY_PAUSE",
			KEY_SCALE => "KEY_SCALE",
			KEY_KPCOMMA => "KEY_KPCOMMA",
			KEY_HANGEUL => "KEY_HANGEUL",
			KEY_HANGUEL => "KEY_HANGUEL",
			KEY_HANJA => "KEY_HANJA",
			KEY_YEN => "KEY_YEN",
			KEY_LEFTMETA => "KEY_LEFTMETA",
			KEY_RIGHTMETA => "KEY_RIGHTMETA",
			KEY_COMPOSE => "KEY_COMPOSE",
			KEY_STOP => "KEY_STOP",
			KEY_AGAIN => "KEY_AGAIN",
			KEY_PROPS => "KEY_PROPS",
			KEY_UNDO => "KEY_UNDO",
			KEY_FRONT => "KEY_FRONT",
			KEY_COPY => "KEY_COPY",
			KEY_OPEN => "KEY_OPEN",
			KEY_PASTE => "KEY_PASTE",
			KEY_FIND => "KEY_FIND",
			KEY_CUT => "KEY_CUT",
			KEY_HELP => "KEY_HELP",
			KEY_MENU => "KEY_MENU",
			KEY_CALC => "KEY_CALC",
			KEY_SETUP => "KEY_SETUP",
			KEY_SLEEP => "KEY_SLEEP",
			KEY_WAKEUP => "KEY_WAKEUP",
			KEY_FILE => "KEY_FILE",
			KEY_SENDFILE => "KEY_SENDFILE",
			KEY_DELETEFILE => "KEY_DELETEFILE",
			KEY_XFER => "KEY_XFER",
			KEY_PROG1 => "KEY_PROG1",
			KEY_PROG2 => "KEY_PROG2",
			KEY_WWW => "KEY_WWW",
			KEY_MSDOS => "KEY_MSDOS",
			KEY_COFFEE => "KEY_COFFEE",
			KEY_SCREENLOCK => "KEY_SCREENLOCK",
			KEY_ROTATE_DISPLAY => "KEY_ROTATE_DISPLAY",
			KEY_DIRECTION => "KEY_DIRECTION",
			KEY_CYCLEWINDOWS => "KEY_CYCLEWINDOWS",
			KEY_MAIL => "KEY_MAIL",
			KEY_BOOKMARKS => "KEY_BOOKMARKS",
			KEY_COMPUTER => "KEY_COMPUTER",
			KEY_BACK => "KEY_BACK",
			KEY_FORWARD => "KEY_FORWARD",
			KEY_CLOSECD => "KEY_CLOSECD",
			KEY_EJECTCD => "KEY_EJECTCD",
			KEY_EJECTCLOSECD => "KEY_EJECTCLOSECD",
			KEY_NEXTSONG => "KEY_NEXTSONG",
			KEY_PLAYPAUSE => "KEY_PLAYPAUSE",
			KEY_PREVIOUSSONG => "KEY_PREVIOUSSONG",
			KEY_STOPCD => "KEY_STOPCD",
			KEY_RECORD => "KEY_RECORD",
			KEY_REWIND => "KEY_REWIND",
			KEY_PHONE => "KEY_PHONE",
			KEY_ISO => "KEY_ISO",
			KEY_CONFIG => "KEY_CONFIG",
			KEY_HOMEPAGE => "KEY_HOMEPAGE",
			KEY_REFRESH => "KEY_REFRESH",
			KEY_EXIT => "KEY_EXIT",
			KEY_MOVE => "KEY_MOVE",
			KEY_EDIT => "KEY_EDIT",
			KEY_SCROLLUP => "KEY_SCROLLUP",
			KEY_SCROLLDOWN => "KEY_SCROLLDOWN",
			KEY_KPLEFTPAREN => "KEY_KPLEFTPAREN",
			KEY_KPRIGHTPAREN => "KEY_KPRIGHTPAREN",
			KEY_NEW => "KEY_NEW",
			KEY_REDO => "KEY_REDO",
			KEY_F13 => "KEY_F13",
			KEY_F14 => "KEY_F14",
			KEY_F15 => "KEY_F15",
			KEY_F16 => "KEY_F16",
			KEY_F17 => "KEY_F17",
			KEY_F18 => "KEY_F18",
			KEY_F19 => "KEY_F19",
			KEY_F20 => "KEY_F20",
			KEY_F21 => "KEY_F21",
			KEY_F22 => "KEY_F22",
			KEY_F23 => "KEY_F23",
			KEY_F24 => "KEY_F24",
			KEY_PLAYCD => "KEY_PLAYCD",
			KEY_PAUSECD => "KEY_PAUSECD",
			KEY_PROG3 => "KEY_PROG3",
			KEY_PROG4 => "KEY_PROG4",
			KEY_DASHBOARD => "KEY_DASHBOARD",
			KEY_SUSPEND => "KEY_SUSPEND",
			KEY_CLOSE => "KEY_CLOSE",
			KEY_PLAY => "KEY_PLAY",
			KEY_FASTFORWARD => "KEY_FASTFORWARD",
			KEY_BASSBOOST => "KEY_BASSBOOST",
			KEY_PRINT => "KEY_PRINT",
			KEY_HP => "KEY_HP",
			KEY_CAMERA => "KEY_CAMERA",
			KEY_SOUND => "KEY_SOUND",
			KEY_QUESTION => "KEY_QUESTION",
			KEY_EMAIL => "KEY_EMAIL",
			KEY_CHAT => "KEY_CHAT",
			KEY_SEARCH => "KEY_SEARCH",
			KEY_CONNECT => "KEY_CONNECT",
			KEY_FINANCE => "KEY_FINANCE",
			KEY_SPORT => "KEY_SPORT",
			KEY_SHOP => "KEY_SHOP",
			KEY_ALTERASE => "KEY_ALTERASE",
			KEY_CANCEL => "KEY_CANCEL",
			KEY_BRIGHTNESSDOWN => "KEY_BRIGHTNESSDOWN",
			KEY_BRIGHTNESSUP => "KEY_BRIGHTNESSUP",
			KEY_MEDIA => "KEY_MEDIA",
			KEY_SWITCHVIDEOMODE => "KEY_SWITCHVIDEOMODE",
			KEY_KBDILLUMTOGGLE => "KEY_KBDILLUMTOGGLE",
			KEY_KBDILLUMDOWN => "KEY_KBDILLUMDOWN",
			KEY_KBDILLUMUP => "KEY_KBDILLUMUP",
			KEY_SEND => "KEY_SEND",
			KEY_REPLY => "KEY_REPLY",
			KEY_FORWARDMAIL => "KEY_FORWARDMAIL",
			KEY_SAVE => "KEY_SAVE",
			KEY_DOCUMENTS => "KEY_DOCUMENTS",
			KEY_BATTERY => "KEY_BATTERY",
			KEY_BLUETOOTH => "KEY_BLUETOOTH",
			KEY_WLAN => "KEY_WLAN",
			KEY_UWB => "KEY_UWB",
			KEY_UNKNOWN => "KEY_UNKNOWN",
			KEY_VIDEO_NEXT => "KEY_VIDEO_NEXT",
			KEY_VIDEO_PREV => "KEY_VIDEO_PREV",
			KEY_BRIGHTNESS_CYCLE => "KEY_BRIGHTNESS_CYCLE",
			KEY_BRIGHTNESS_AUTO => "KEY_BRIGHTNESS_AUTO",
			KEY_BRIGHTNESS_ZERO => "KEY_BRIGHTNESS_ZERO",
			KEY_DISPLAY_OFF => "KEY_DISPLAY_OFF",
			KEY_WWAN => "KEY_WWAN",
			KEY_WIMAX => "KEY_WIMAX",
			KEY_RFKILL => "KEY_RFKILL",
			KEY_MICMUTE => "KEY_MICMUTE",
			BTN_MISC => "BTN_MISC",
			BTN_0 => "BTN_0",
			BTN_1 => "BTN_1",
			BTN_2 => "BTN_2",
			BTN_3 => "BTN_3",
			BTN_4 => "BTN_4",
			BTN_5 => "BTN_5",
			BTN_6 => "BTN_6",
			BTN_7 => "BTN_7",
			BTN_8 => "BTN_8",
			BTN_9 => "BTN_9",
			BTN_MOUSE => "BTN_MOUSE",
			BTN_LEFT => "BTN_LEFT",
			BTN_RIGHT => "BTN_RIGHT",
			BTN_MIDDLE => "BTN_MIDDLE",
			BTN_SIDE => "BTN_SIDE",
			BTN_EXTRA => "BTN_EXTRA",
			BTN_FORWARD => "BTN_FORWARD",
			BTN_BACK => "BTN_BACK",
			BTN_TASK => "BTN_TASK",
			BTN_JOYSTICK => "BTN_JOYSTICK",
			BTN_TRIGGER => "BTN_TRIGGER",
			BTN_THUMB => "BTN_THUMB",
			BTN_THUMB2 => "BTN_THUMB2",
			BTN_TOP => "BTN_TOP",
			BTN_TOP2 => "BTN_TOP2",
			BTN_PINKIE => "BTN_PINKIE",
			BTN_BASE => "BTN_BASE",
			BTN_BASE2 => "BTN_BASE2",
			BTN_BASE3 => "BTN_BASE3",
			BTN_BASE4 => "BTN_BASE4",
			BTN_BASE5 => "BTN_BASE5",
			BTN_BASE6 => "BTN_BASE6",
			BTN_DEAD => "BTN_DEAD",
			BTN_GAMEPAD => "BTN_GAMEPAD",
			BTN_SOUTH => "BTN_SOUTH",
			BTN_A => "BTN_A",
			BTN_EAST => "BTN_EAST",
			BTN_B => "BTN_B",
			BTN_C => "BTN_C",
			BTN_NORTH => "BTN_NORTH",
			BTN_X => "BTN_X",
			BTN_WEST => "BTN_WEST",
			BTN_Y => "BTN_Y",
			BTN_Z => "BTN_Z",
			BTN_TL => "BTN_TL",
			BTN_TR => "BTN_TR",
			BTN_TL2 => "BTN_TL2",
			BTN_TR2 => "BTN_TR2",
			BTN_SELECT => "BTN_SELECT",
			BTN_START => "BTN_START",
			BTN_MODE => "BTN_MODE",
			BTN_THUMBL => "BTN_THUMBL",
			BTN_THUMBR => "BTN_THUMBR",
			BTN_DIGI => "BTN_DIGI",
			BTN_TOOL_PEN => "BTN_TOOL_PEN",
			BTN_TOOL_RUBBER => "BTN_TOOL_RUBBER",
			BTN_TOOL_BRUSH => "BTN_TOOL_BRUSH",
			BTN_TOOL_PENCIL => "BTN_TOOL_PENCIL",
			BTN_TOOL_AIRBRUSH => "BTN_TOOL_AIRBRUSH",
			BTN_TOOL_FINGER => "BTN_TOOL_FINGER",
			BTN_TOOL_MOUSE => "BTN_TOOL_MOUSE",
			BTN_TOOL_LENS => "BTN_TOOL_LENS",
			BTN_TOOL_QUINTTAP => "BTN_TOOL_QUINTTAP",
			BTN_STYLUS3 => "BTN_STYLUS3",
			BTN_TOUCH => "BTN_TOUCH",
			BTN_STYLUS => "BTN_STYLUS",
			BTN_STYLUS2 => "BTN_STYLUS2",
			BTN_TOOL_DOUBLETAP => "BTN_TOOL_DOUBLETAP",
			BTN_TOOL_TRIPLETAP => "BTN_TOOL_TRIPLETAP",
			BTN_TOOL_QUADTAP => "BTN_TOOL_QUADTAP",
			BTN_WHEEL => "BTN_WHEEL",
			BTN_GEAR_DOWN => "BTN_GEAR_DOWN",
			BTN_GEAR_UP => "BTN_GEAR_UP",
			KEY_OK => "KEY_OK",
			KEY_SELECT => "KEY_SELECT",
			KEY_GOTO => "KEY_GOTO",
			KEY_CLEAR => "KEY_CLEAR",
			KEY_POWER2 => "KEY_POWER2",
			KEY_OPTION => "KEY_OPTION",
			KEY_INFO => "KEY_INFO",
			KEY_TIME => "KEY_TIME",
			KEY_VENDOR => "KEY_VENDOR",
			KEY_ARCHIVE => "KEY_ARCHIVE",
			KEY_PROGRAM => "KEY_PROGRAM",
			KEY_CHANNEL => "KEY_CHANNEL",
			KEY_FAVORITES => "KEY_FAVORITES",
			KEY_EPG => "KEY_EPG",
			KEY_PVR => "KEY_PVR",
			KEY_MHP => "KEY_MHP",
			KEY_LANGUAGE => "KEY_LANGUAGE",
			KEY_TITLE => "KEY_TITLE",
			KEY_SUBTITLE => "KEY_SUBTITLE",
			KEY_ANGLE => "KEY_ANGLE",
			KEY_FULL_SCREEN => "KEY_FULL_SCREEN",
			KEY_ZOOM => "KEY_ZOOM",
			KEY_MODE => "KEY_MODE",
			KEY_KEYBOARD => "KEY_KEYBOARD",
			KEY_ASPECT_RATIO => "KEY_ASPECT_RATIO",
			KEY_SCREEN => "KEY_SCREEN",
			KEY_PC => "KEY_PC",
			KEY_TV => "KEY_TV",
			KEY_TV2 => "KEY_TV2",
			KEY_VCR => "KEY_VCR",
			KEY_VCR2 => "KEY_VCR2",
			KEY_SAT => "KEY_SAT",
			KEY_SAT2 => "KEY_SAT2",
			KEY_CD => "KEY_CD",
			KEY_TAPE => "KEY_TAPE",
			KEY_RADIO => "KEY_RADIO",
			KEY_TUNER => "KEY_TUNER",
			KEY_PLAYER => "KEY_PLAYER",
			KEY_TEXT => "KEY_TEXT",
			KEY_DVD => "KEY_DVD",
			KEY_AUX => "KEY_AUX",
			KEY_MP3 => "KEY_MP3",
			KEY_AUDIO => "KEY_AUDIO",
			KEY_VIDEO => "KEY_VIDEO",
			KEY_DIRECTORY => "KEY_DIRECTORY",
			KEY_LIST => "KEY_LIST",
			KEY_MEMO => "KEY_MEMO",
			KEY_CALENDAR => "KEY_CALENDAR",
			KEY_RED => "KEY_RED",
			KEY_GREEN => "KEY_GREEN",
			KEY_YELLOW => "KEY_YELLOW",
			KEY_BLUE => "KEY_BLUE",
			KEY_CHANNELUP => "KEY_CHANNELUP",
			KEY_CHANNELDOWN => "KEY_CHANNELDOWN",
			KEY_FIRST => "KEY_FIRST",
			KEY_LAST => "KEY_LAST",
			KEY_AB => "KEY_AB",
			KEY_NEXT => "KEY_NEXT",
			KEY_RESTART => "KEY_RESTART",
			KEY_SLOW => "KEY_SLOW",
			KEY_SHUFFLE => "KEY_SHUFFLE",
			KEY_BREAK => "KEY_BREAK",
			KEY_PREVIOUS => "KEY_PREVIOUS",
			KEY_DIGITS => "KEY_DIGITS",
			KEY_TEEN => "KEY_TEEN",
			KEY_TWEN => "KEY_TWEN",
			KEY_VIDEOPHONE => "KEY_VIDEOPHONE",
			KEY_GAMES => "KEY_GAMES",
			KEY_ZOOMIN => "KEY_ZOOMIN",
			KEY_ZOOMOUT => "KEY_ZOOMOUT",
			KEY_ZOOMRESET => "KEY_ZOOMRESET",
			KEY_WORDPROCESSOR => "KEY_WORDPROCESSOR",
			KEY_EDITOR => "KEY_EDITOR",
			KEY_SPREADSHEET => "KEY_SPREADSHEET",
			KEY_GRAPHICSEDITOR => "KEY_GRAPHICSEDITOR",
			KEY_PRESENTATION => "KEY_PRESENTATION",
			KEY_DATABASE => "KEY_DATABASE",
			KEY_NEWS => "KEY_NEWS",
			KEY_VOICEMAIL => "KEY_VOICEMAIL",
			KEY_ADDRESSBOOK => "KEY_ADDRESSBOOK",
			KEY_MESSENGER => "KEY_MESSENGER",
			KEY_DISPLAYTOGGLE => "KEY_DISPLAYTOGGLE",
			KEY_BRIGHTNESS_TOGGLE => "KEY_BRIGHTNESS_TOGGLE",
			KEY_SPELLCHECK => "KEY_SPELLCHECK",
			KEY_LOGOFF => "KEY_LOGOFF",
			KEY_DOLLAR => "KEY_DOLLAR",
			KEY_EURO => "KEY_EURO",
			KEY_FRAMEBACK => "KEY_FRAMEBACK",
			KEY_FRAMEFORWARD => "KEY_FRAMEFORWARD",
			KEY_CONTEXT_MENU => "KEY_CONTEXT_MENU",
			KEY_MEDIA_REPEAT => "KEY_MEDIA_REPEAT",
			KEY_10CHANNELSUP => "KEY_10CHANNELSUP",
			KEY_10CHANNELSDOWN => "KEY_10CHANNELSDOWN",
			KEY_IMAGES => "KEY_IMAGES",
			KEY_DEL_EOL => "KEY_DEL_EOL",
			KEY_DEL_EOS => "KEY_DEL_EOS",
			KEY_INS_LINE => "KEY_INS_LINE",
			KEY_DEL_LINE => "KEY_DEL_LINE",
			KEY_FN => "KEY_FN",
			KEY_FN_ESC => "KEY_FN_ESC",
			KEY_FN_F1 => "KEY_FN_F1",
			KEY_FN_F2 => "KEY_FN_F2",
			KEY_FN_F3 => "KEY_FN_F3",
			KEY_FN_F4 => "KEY_FN_F4",
			KEY_FN_F5 => "KEY_FN_F5",
			KEY_FN_F6 => "KEY_FN_F6",
			KEY_FN_F7 => "KEY_FN_F7",
			KEY_FN_F8 => "KEY_FN_F8",
			KEY_FN_F9 => "KEY_FN_F9",
			KEY_FN_F10 => "KEY_FN_F10",
			KEY_FN_F11 => "KEY_FN_F11",
			KEY_FN_F12 => "KEY_FN_F12",
			KEY_FN_1 => "KEY_FN_1",
			KEY_FN_2 => "KEY_FN_2",
			KEY_FN_D => "KEY_FN_D",
			KEY_FN_E => "KEY_FN_E",
			KEY_FN_F => "KEY_FN_F",
			KEY_FN_S => "KEY_FN_S",
			KEY_FN_B => "KEY_FN_B",
			KEY_BRL_DOT1 => "KEY_BRL_DOT1",
			KEY_BRL_DOT2 => "KEY_BRL_DOT2",
			KEY_BRL_DOT3 => "KEY_BRL_DOT3",
			KEY_BRL_DOT4 => "KEY_BRL_DOT4",
			KEY_BRL_DOT5 => "KEY_BRL_DOT5",
			KEY_BRL_DOT6 => "KEY_BRL_DOT6",
			KEY_BRL_DOT7 => "KEY_BRL_DOT7",
			KEY_BRL_DOT8 => "KEY_BRL_DOT8",
			KEY_BRL_DOT9 => "KEY_BRL_DOT9",
			KEY_BRL_DOT10 => "KEY_BRL_DOT10",
			KEY_NUMERIC_0 => "KEY_NUMERIC_0",
			KEY_NUMERIC_1 => "KEY_NUMERIC_1",
			KEY_NUMERIC_2 => "KEY_NUMERIC_2",
			KEY_NUMERIC_3 => "KEY_NUMERIC_3",
			KEY_NUMERIC_4 => "KEY_NUMERIC_4",
			KEY_NUMERIC_5 => "KEY_NUMERIC_5",
			KEY_NUMERIC_6 => "KEY_NUMERIC_6",
			KEY_NUMERIC_7 => "KEY_NUMERIC_7",
			KEY_NUMERIC_8 => "KEY_NUMERIC_8",
			KEY_NUMERIC_9 => "KEY_NUMERIC_9",
			KEY_NUMERIC_STAR => "KEY_NUMERIC_STAR",
			KEY_NUMERIC_POUND => "KEY_NUMERIC_POUND",
			KEY_NUMERIC_A => "KEY_NUMERIC_A",
			KEY_NUMERIC_B => "KEY_NUMERIC_B",
			KEY_NUMERIC_C => "KEY_NUMERIC_C",
			KEY_NUMERIC_D => "KEY_NUMERIC_D",
			KEY_CAMERA_FOCUS => "KEY_CAMERA_FOCUS",
			KEY_WPS_BUTTON => "KEY_WPS_BUTTON",
			KEY_TOUCHPAD_TOGGLE => "KEY_TOUCHPAD_TOGGLE",
			KEY_TOUCHPAD_ON => "KEY_TOUCHPAD_ON",
			KEY_TOUCHPAD_OFF => "KEY_TOUCHPAD_OFF",
			KEY_CAMERA_ZOOMIN => "KEY_CAMERA_ZOOMIN",
			KEY_CAMERA_ZOOMOUT => "KEY_CAMERA_ZOOMOUT",
			KEY_CAMERA_UP => "KEY_CAMERA_UP",
			KEY_CAMERA_DOWN => "KEY_CAMERA_DOWN",
			KEY_CAMERA_LEFT => "KEY_CAMERA_LEFT",
			KEY_CAMERA_RIGHT => "KEY_CAMERA_RIGHT",
			KEY_ATTENDANT_ON => "KEY_ATTENDANT_ON",
			KEY_ATTENDANT_OFF => "KEY_ATTENDANT_OFF",
			KEY_ATTENDANT_TOGGLE => "KEY_ATTENDANT_TOGGLE",
			KEY_LIGHTS_TOGGLE => "KEY_LIGHTS_TOGGLE",
			BTN_DPAD_UP => "BTN_DPAD_UP",
			BTN_DPAD_DOWN => "BTN_DPAD_DOWN",
			BTN_DPAD_LEFT => "BTN_DPAD_LEFT",
			BTN_DPAD_RIGHT => "BTN_DPAD_RIGHT",
			KEY_ALS_TOGGLE => "KEY_ALS_TOGGLE",
			KEY_ROTATE_LOCK_TOGGLE => "KEY_ROTATE_LOCK_TOGGLE",
			KEY_BUTTONCONFIG => "KEY_BUTTONCONFIG",
			KEY_TASKMANAGER => "KEY_TASKMANAGER",
			KEY_JOURNAL => "KEY_JOURNAL",
			KEY_CONTROLPANEL => "KEY_CONTROLPANEL",
			KEY_APPSELECT => "KEY_APPSELECT",
			KEY_SCREENSAVER => "KEY_SCREENSAVER",
			KEY_VOICECOMMAND => "KEY_VOICECOMMAND",
			KEY_ASSISTANT => "KEY_ASSISTANT",
			KEY_KBD_LAYOUT_NEXT => "KEY_KBD_LAYOUT_NEXT",
			KEY_BRIGHTNESS_MIN => "KEY_BRIGHTNESS_MIN",
			KEY_KBDINPUTASSIST_PREV => "KEY_KBDINPUTASSIST_PREV",
			KEY_KBDINPUTASSIST_NEXT => "KEY_KBDINPUTASSIST_NEXT",
			KEY_KBDINPUTASSIST_PREVGROUP => "KEY_KBDINPUTASSIST_PREVGROUP",
			KEY_KBDINPUTASSIST_NEXTGROUP => "KEY_KBDINPUTASSIST_NEXTGROUP",
			KEY_KBDINPUTASSIST_ACCEPT => "KEY_KBDINPUTASSIST_ACCEPT",
			KEY_KBDINPUTASSIST_CANCEL => "KEY_KBDINPUTASSIST_CANCEL",
			KEY_RIGHT_UP => "KEY_RIGHT_UP",
			KEY_RIGHT_DOWN => "KEY_RIGHT_DOWN",
			KEY_LEFT_UP => "KEY_LEFT_UP",
			KEY_LEFT_DOWN => "KEY_LEFT_DOWN",
			KEY_ROOT_MENU => "KEY_ROOT_MENU",
			KEY_MEDIA_TOP_MENU => "KEY_MEDIA_TOP_MENU",
			KEY_NUMERIC_11 => "KEY_NUMERIC_11",
			KEY_NUMERIC_12 => "KEY_NUMERIC_12",
			KEY_AUDIO_DESC => "KEY_AUDIO_DESC",
			KEY_3D_MODE => "KEY_3D_MODE",
			KEY_NEXT_FAVORITE => "KEY_NEXT_FAVORITE",
			KEY_STOP_RECORD => "KEY_STOP_RECORD",
			KEY_PAUSE_RECORD => "KEY_PAUSE_RECORD",
			KEY_VOD => "KEY_VOD",
			KEY_UNMUTE => "KEY_UNMUTE",
			KEY_FASTREVERSE => "KEY_FASTREVERSE",
			KEY_SLOWREVERSE => "KEY_SLOWREVERSE",
			KEY_DATA => "KEY_DATA",
			KEY_ONSCREEN_KEYBOARD => "KEY_ONSCREEN_KEYBOARD",
			KEY_PRIVACY_SCREEN_TOGGLE => "KEY_PRIVACY_SCREEN_TOGGLE",
			KEY_SELECTIVE_SCREENSHOT => "KEY_SELECTIVE_SCREENSHOT",
			KEY_MACRO1 => "KEY_MACRO1",
			KEY_MACRO2 => "KEY_MACRO2",
			KEY_MACRO3 => "KEY_MACRO3",
			KEY_MACRO4 => "KEY_MACRO4",
			KEY_MACRO5 => "KEY_MACRO5",
			KEY_MACRO6 => "KEY_MACRO6",
			KEY_MACRO7 => "KEY_MACRO7",
			KEY_MACRO8 => "KEY_MACRO8",
			KEY_MACRO9 => "KEY_MACRO9",
			KEY_MACRO10 => "KEY_MACRO10",
			KEY_MACRO11 => "KEY_MACRO11",
			KEY_MACRO12 => "KEY_MACRO12",
			KEY_MACRO13 => "KEY_MACRO13",
			KEY_MACRO14 => "KEY_MACRO14",
			KEY_MACRO15 => "KEY_MACRO15",
			KEY_MACRO16 => "KEY_MACRO16",
			KEY_MACRO17 => "KEY_MACRO17",
			KEY_MACRO18 => "KEY_MACRO18",
			KEY_MACRO19 => "KEY_MACRO19",
			KEY_MACRO20 => "KEY_MACRO20",
			KEY_MACRO21 => "KEY_MACRO21",
			KEY_MACRO22 => "KEY_MACRO22",
			KEY_MACRO23 => "KEY_MACRO23",
			KEY_MACRO24 => "KEY_MACRO24",
			KEY_MACRO25 => "KEY_MACRO25",
			KEY_MACRO26 => "KEY_MACRO26",
			KEY_MACRO27 => "KEY_MACRO27",
			KEY_MACRO28 => "KEY_MACRO28",
			KEY_MACRO29 => "KEY_MACRO29",
			KEY_MACRO30 => "KEY_MACRO30",
			KEY_MACRO_RECORD_START => "KEY_MACRO_RECORD_START",
			KEY_MACRO_RECORD_STOP => "KEY_MACRO_RECORD_STOP",
			KEY_MACRO_PRESET_CYCLE => "KEY_MACRO_PRESET_CYCLE",
			KEY_MACRO_PRESET1 => "KEY_MACRO_PRESET1",
			KEY_MACRO_PRESET2 => "KEY_MACRO_PRESET2",
			KEY_MACRO_PRESET3 => "KEY_MACRO_PRESET3",
			KEY_KBD_LCD_MENU1 => "KEY_KBD_LCD_MENU1",
			KEY_KBD_LCD_MENU2 => "KEY_KBD_LCD_MENU2",
			KEY_KBD_LCD_MENU3 => "KEY_KBD_LCD_MENU3",
			KEY_KBD_LCD_MENU4 => "KEY_KBD_LCD_MENU4",
			KEY_KBD_LCD_MENU5 => "KEY_KBD_LCD_MENU5",
			BTN_TRIGGER_HAPPY => "BTN_TRIGGER_HAPPY",
			BTN_TRIGGER_HAPPY1 => "BTN_TRIGGER_HAPPY1",
			BTN_TRIGGER_HAPPY2 => "BTN_TRIGGER_HAPPY2",
			BTN_TRIGGER_HAPPY3 => "BTN_TRIGGER_HAPPY3",
			BTN_TRIGGER_HAPPY4 => "BTN_TRIGGER_HAPPY4",
			BTN_TRIGGER_HAPPY5 => "BTN_TRIGGER_HAPPY5",
			BTN_TRIGGER_HAPPY6 => "BTN_TRIGGER_HAPPY6",
			BTN_TRIGGER_HAPPY7 => "BTN_TRIGGER_HAPPY7",
			BTN_TRIGGER_HAPPY8 => "BTN_TRIGGER_HAPPY8",
			BTN_TRIGGER_HAPPY9 => "BTN_TRIGGER_HAPPY9",
			BTN_TRIGGER_HAPPY10 => "BTN_TRIGGER_HAPPY10",
			BTN_TRIGGER_HAPPY11 => "BTN_TRIGGER_HAPPY11",
			BTN_TRIGGER_HAPPY12 => "BTN_TRIGGER_HAPPY12",
			BTN_TRIGGER_HAPPY13 => "BTN_TRIGGER_HAPPY13",
			BTN_TRIGGER_HAPPY14 => "BTN_TRIGGER_HAPPY14",
			BTN_TRIGGER_HAPPY15 => "BTN_TRIGGER_HAPPY15",
			BTN_TRIGGER_HAPPY16 => "BTN_TRIGGER_HAPPY16",
			BTN_TRIGGER_HAPPY17 => "BTN_TRIGGER_HAPPY17",
			BTN_TRIGGER_HAPPY18 => "BTN_TRIGGER_HAPPY18",
			BTN_TRIGGER_HAPPY19 => "BTN_TRIGGER_HAPPY19",
			BTN_TRIGGER_HAPPY20 => "BTN_TRIGGER_HAPPY20",
			BTN_TRIGGER_HAPPY21 => "BTN_TRIGGER_HAPPY21",
			BTN_TRIGGER_HAPPY22 => "BTN_TRIGGER_HAPPY22",
			BTN_TRIGGER_HAPPY23 => "BTN_TRIGGER_HAPPY23",
			BTN_TRIGGER_HAPPY24 => "BTN_TRIGGER_HAPPY24",
			BTN_TRIGGER_HAPPY25 => "BTN_TRIGGER_HAPPY25",
			BTN_TRIGGER_HAPPY26 => "BTN_TRIGGER_HAPPY26",
			BTN_TRIGGER_HAPPY27 => "BTN_TRIGGER_HAPPY27",
			BTN_TRIGGER_HAPPY28 => "BTN_TRIGGER_HAPPY28",
			BTN_TRIGGER_HAPPY29 => "BTN_TRIGGER_HAPPY29",
			BTN_TRIGGER_HAPPY30 => "BTN_TRIGGER_HAPPY30",
			BTN_TRIGGER_HAPPY31 => "BTN_TRIGGER_HAPPY31",
			BTN_TRIGGER_HAPPY32 => "BTN_TRIGGER_HAPPY32",
			BTN_TRIGGER_HAPPY33 => "BTN_TRIGGER_HAPPY33",
			BTN_TRIGGER_HAPPY34 => "BTN_TRIGGER_HAPPY34",
			BTN_TRIGGER_HAPPY35 => "BTN_TRIGGER_HAPPY35",
			BTN_TRIGGER_HAPPY36 => "BTN_TRIGGER_HAPPY36",
			BTN_TRIGGER_HAPPY37 => "BTN_TRIGGER_HAPPY37",
			BTN_TRIGGER_HAPPY38 => "BTN_TRIGGER_HAPPY38",
			BTN_TRIGGER_HAPPY39 => "BTN_TRIGGER_HAPPY39",
			BTN_TRIGGER_HAPPY40 => "BTN_TRIGGER_HAPPY40",
			KEY_MIN_INTERESTING => "KEY_MIN_INTERESTING",
			_ => "",
		}
		EV_REL => match code {
			REL_X => "REL_X",
			REL_Y => "REL_Y",
			REL_Z => "REL_Z",
			REL_RX => "REL_RX",
			REL_RY => "REL_RY",
			REL_RZ => "REL_RZ",
			REL_HWHEEL => "REL_HWHEEL",
			REL_DIAL => "REL_DIAL",
			REL_WHEEL => "REL_WHEEL",
			REL_MISC => "REL_MISC",
			REL_RESERVED => "REL_RESERVED",
			REL_WHEEL_HI_RES => "REL_WHEEL_HI_RES",
			REL_HWHEEL_HI_RES => "REL_HWHEEL_HI_RES",
			_ => "",
		}
		EV_ABS => match code {
			ABS_X => "ABS_X",
			ABS_Y => "ABS_Y",
			ABS_Z => "ABS_Z",
			ABS_RX => "ABS_RX",
			ABS_RY => "ABS_RY",
			ABS_RZ => "ABS_RZ",
			ABS_THROTTLE => "ABS_THROTTLE",
			ABS_RUDDER => "ABS_RUDDER",
			ABS_WHEEL => "ABS_WHEEL",
			ABS_GAS => "ABS_GAS",
			ABS_BRAKE => "ABS_BRAKE",
			ABS_HAT0X => "ABS_HAT0X",
			ABS_HAT0Y => "ABS_HAT0Y",
			ABS_HAT1X => "ABS_HAT1X",
			ABS_HAT1Y => "ABS_HAT1Y",
			ABS_HAT2X => "ABS_HAT2X",
			ABS_HAT2Y => "ABS_HAT2Y",
			ABS_HAT3X => "ABS_HAT3X",
			ABS_HAT3Y => "ABS_HAT3Y",
			ABS_PRESSURE => "ABS_PRESSURE",
			ABS_DISTANCE => "ABS_DISTANCE",
			ABS_TILT_X => "ABS_TILT_X",
			ABS_TILT_Y => "ABS_TILT_Y",
			ABS_TOOL_WIDTH => "ABS_TOOL_WIDTH",
			ABS_VOLUME => "ABS_VOLUME",
			ABS_MISC => "ABS_MISC",
			ABS_RESERVED => "ABS_RESERVED",
			ABS_MT_SLOT => "ABS_MT_SLOT",
			ABS_MT_TOUCH_MAJOR => "ABS_MT_TOUCH_MAJOR",
			ABS_MT_TOUCH_MINOR => "ABS_MT_TOUCH_MINOR",
			ABS_MT_WIDTH_MAJOR => "ABS_MT_WIDTH_MAJOR",
			ABS_MT_WIDTH_MINOR => "ABS_MT_WIDTH_MINOR",
			ABS_MT_ORIENTATION => "ABS_MT_ORIENTATION",
			ABS_MT_POSITION_X => "ABS_MT_POSITION_X",
			ABS_MT_POSITION_Y => "ABS_MT_POSITION_Y",
			ABS_MT_TOOL_TYPE => "ABS_MT_TOOL_TYPE",
			ABS_MT_BLOB_ID => "ABS_MT_BLOB_ID",
			ABS_MT_TRACKING_ID => "ABS_MT_TRACKING_ID",
			ABS_MT_PRESSURE => "ABS_MT_PRESSURE",
			ABS_MT_DISTANCE => "ABS_MT_DISTANCE",
			ABS_MT_TOOL_X => "ABS_MT_TOOL_X",
			ABS_MT_TOOL_Y => "ABS_MT_TOOL_Y",
			_ => "",
		}
		EV_MSC => match code {
			MSC_SERIAL => "MSC_SERIAL",
			MSC_PULSELED => "MSC_PULSELED",
			MSC_GESTURE => "MSC_GESTURE",
			MSC_RAW => "MSC_RAW",
			MSC_SCAN => "MSC_SCAN",
			MSC_TIMESTAMP => "MSC_TIMESTAMP",
			_ => "",
		}
		EV_SW => match code {
			SW_LID => "SW_LID",
			SW_TABLET_MODE => "SW_TABLET_MODE",
			SW_HEADPHONE_INSERT => "SW_HEADPHONE_INSERT",
			SW_RFKILL_ALL => "SW_RFKILL_ALL",
			SW_RADIO => "SW_RADIO",
			SW_MICROPHONE_INSERT => "SW_MICROPHONE_INSERT",
			SW_DOCK => "SW_DOCK",
			SW_LINEOUT_INSERT => "SW_LINEOUT_INSERT",
			SW_JACK_PHYSICAL_INSERT => "SW_JACK_PHYSICAL_INSERT",
			SW_VIDEOOUT_INSERT => "SW_VIDEOOUT_INSERT",
			SW_CAMERA_LENS_COVER => "SW_CAMERA_LENS_COVER",
			SW_KEYPAD_SLIDE => "SW_KEYPAD_SLIDE",
			SW_FRONT_PROXIMITY => "SW_FRONT_PROXIMITY",
			SW_ROTATE_LOCK => "SW_ROTATE_LOCK",
			SW_LINEIN_INSERT => "SW_LINEIN_INSERT",
			SW_MUTE_DEVICE => "SW_MUTE_DEVICE",
			SW_PEN_INSERTED => "SW_PEN_INSERTED",
			SW_MACHINE_COVER => "SW_MACHINE_COVER",
			_ => "",
		}
		EV_LED => match code {
			LED_NUML => "LED_NUML",
			LED_CAPSL => "LED_CAPSL",
			LED_SCROLLL => "LED_SCROLLL",
			LED_COMPOSE => "LED_COMPOSE",
			LED_KANA => "LED_KANA",
			LED_SLEEP => "LED_SLEEP",
			LED_SUSPEND => "LED_SUSPEND",
			LED_MUTE => "LED_MUTE",
			LED_MISC => "LED_MISC",
			LED_MAIL => "LED_MAIL",
			LED_CHARGING => "LED_CHARGING",
			_ => "",
		}
		EV_SND => match code {
			SND_CLICK => "SND_CLICK",
			SND_BELL => "SND_BELL",
			SND_TONE => "SND_TONE",
			_ => "",
		}
		EV_REP => match code {
			REP_DELAY => "REP_DELAY",
			REP_PERIOD => "REP_PERIOD",
			_ => "",
		}
		_ => "",
	}
}
