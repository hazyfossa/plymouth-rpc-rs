// NOTE: on an actual, full protocol implementation there should be a single source of method definitions
// and Client and ServerTemplate should be derived for it.
// For now, though, this is not the case, so this is commented out.

// use enum_repr::EnumRepr;

// #[EnumRepr(type = "char")]
// pub enum Method {
//     Ping = 'P',
//     Update = 'U',
//     ChangeMode = 'C',
//     SystemUpdate = 'u',
//     SystemInitialized = 'S',
//     Deactivate = 'D',
//     Reactivate = 'r',
//     Quit = 'Q',
//     Reload = 'l',
//     Password = '*',
//     CachedPassword = 'c',
//     Question = 'W',
//     ShowMessage = 'M',
//     HideMessage = 'm',
//     Keystroke = 'K',
//     KeystrokeRemove = 'L',
//     ProgressPause = 'A',
//     ProgressUnpause = 'a',
//     ShowSplash = '$',
//     HideSplash = 'H',
//     Newroot = 'R',
//     HasActiveVt = 'V',
//     Error = '!',
// }
