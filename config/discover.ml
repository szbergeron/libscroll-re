type os =
    | Windows
    | Mac
    | Linux
    | Unknown


let libpath = "-L../target/debug"

let flags =
    match get_os with
    | Linux -> []
        @ cclib(libpath)
    | _ => []
