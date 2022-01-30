initSidebarItems({"enum":[["ConnError","Error type that is returned by `Connection::has_error`."],["Error","The general error type for Rust-XCB."],["Event","Unified Event type from the X server."],["EventQueueOwner","Determines whether Xlib or XCB owns the event queue of [`Connection`]."],["Extension","Refers to a X protocol extension."],["Lat1Error","Error that can produce Latin-1 string operations"],["ProtocolError","A protocol error issued from the X server"]],"fn":[["parse_display","Parses a display string in the form documented by X (7x)."]],"mod":[["bigreq","The `BIG-REQUESTS` extension."],["composite","The `Composite` X extension."],["damage","The `DAMAGE` X extension."],["dpms","The `DPMS` X extension."],["dri2","The `DRI2` X extension."],["dri3","The `DRI3` X extension."],["ffi","Module for Foreign Function Interface bindings."],["ge","The `Generic Event Extension` X extension."],["glx","The `GLX` X extension."],["present","The `Present` X extension."],["randr","The `RANDR` X extension."],["record","The `RECORD` X extension."],["render","The `RENDER` X extension."],["res","The `X-Resource` X extension."],["screensaver","The `MIT-SCREEN-SAVER` X extension."],["shape","The `SHAPE` X extension."],["shm","The `MIT-SHM` X extension."],["sync","The `SYNC` X extension."],["x","The core X protocol definitions"],["xc_misc","The `XC-MISC` extension."],["xevie","The `XEVIE` X extension."],["xf86dri","The `XFree86-DRI` X extension."],["xf86vidmode","The `XFree86-VidModeExtension` X extension."],["xfixes","The `XFIXES` X extension."],["xinerama","The `XINERAMA` X extension."],["xinput","The `XInputExtension` X extension."],["xkb","The `XKEYBOARD` X extension."],["xprint","The `XpExtension` X extension."],["xselinux","The `SELinux` X extension."],["xtest","The `XTEST` X extension."],["xv","The `XVideo` X extension."],["xvmc","The `XVideo-MotionCompensation` X extension."]],"struct":[["AuthInfo","Container for authentication information to connect to the X server"],["Connection","`Connection` is the central object of XCB."],["DisplayInfo","Display info returned by [`parse_display`]"],["ExtensionData","Extension data as returned by each extensions `get_extension_data`."],["Lat1Str","A slice to a Latin-1 (aka. ISO 8859-1) string."],["Lat1StrF","Latin-1 (aka. ISO 8859-1) of fixed size"],["Lat1String","A struct owning a Latin-1 (aka. ISO 8859-1) string."],["SpecialEventId","A struct that serve as an identifier for internal special queue in XCB"],["UnknownEvent","an event was not recognized as part of the core protocol or any enabled extension"],["VoidCookie","The default cookie type returned by void-requests."],["VoidCookieChecked","The checked cookie type returned by void-requests."]],"trait":[["BaseError","A trait to designate base protocol errors."],["BaseEvent","Trait for base events (aka. non GE_GENERIC events)"],["Cookie","General trait for cookies returned by requests."],["CookieChecked","A marker trait for a cookie that allows synchronized error checking."],["CookieWithReplyChecked","A trait for checked cookies of requests that send a reply."],["CookieWithReplyUnchecked","A trait for unchecked cookies of requests that send a reply."],["GeEvent","A trait for GE_GENERIC events"],["Raw","Trait for types that own a C allocated pointer and are represented by the data pointed to."],["RawRequest","Trait implemented by all requests to send the serialized data over the wire."],["Reply","Trait for request replies"],["Request","Trait implemented by requests types."],["RequestWithReply","Trait for requests that return a reply."],["RequestWithoutReply","Marker trait for requests that do not return a reply."],["Xid","A X resource trait"],["XidNew","Trait for X resources that can be created directly from `Connection::generate_id`"]],"type":[["ConnResult","The result type associated with [ConnError]."],["ProtocolResult","The result type associated with [ProtocolError]."],["Result","The general result type for Rust-XCB."]]});