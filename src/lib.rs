pub fn rgb(r: i32, g: i32, b: i32) -> String {
    // format as 2-char with leading zero if necessary
    // then use clamp to keep in range 0-255
    format!("{:02X}{:02X}{:02X}", 
        r.clamp(0,255),
        g.clamp(0,255),
        b.clamp(0,255)
    )

}