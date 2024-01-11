use inline_colorization::*;
use std::io;
use std::io::prelude::*;

// My dummy logo
static MADMONKEY: &str = r#"
     __
w  c(..)o   (
 \__(-)    __)
     /\   ( 
    /(_)___) 
    w /| 
     | \ 
    m  m 
"#;

// About
static AUTHOR: &str = r#"
Author:   Frederico Sales 
E-mail:   <frederico@fredericosales.eng.br>
2024

"#;

fn main() {
    // A liitle of art.
    println!("{style_bold}{color_yellow}{}{color_reset}{style_reset}\n", MADMONKEY);

    // Mode key
    println!("{style_bold}mode key: Ruindows key.{style_reset}\n\n");

    // Mode keys
    println!("{style_bold}{color_green}[key bind]{color_reset}              {color_cyan}[action]{color_reset}{style_reset}");
    println!("{style_bold}{color_green}F1                      {color_cyan}- Show this help.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + return       {color_cyan}- Open kitty term.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + C            {color_cyan}- Kill active window.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + M            {color_cyan}- Exit Hyprland.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + E            {color_cyan}- Open thunar file manager.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + V            {color_cyan}- Toggle float mode.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + SPACE        {color_cyan}- Open wofi in drun mode.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + R            {color_cyan}- Open wofi in run mode.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + P            {color_cyan}- Toggle pseudo mode.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + J            {color_cyan}- Toogle split mode.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + F            {color_cyan}- Open the default web browser.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + T            {color_cyan}- Open notify center.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + SHIFT + M    {color_cyan}- Open sound control center.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}PrtSc                   {color_cyan}- Take a print screen.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key + PrtSc        {color_cyan}- Take a print screen on select mode.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key arrow keys     {color_cyan}- Toggle between windows focus.{color_reset}{style_reset}");
    println!("{style_bold}{color_green}mode key number keys    {color_cyan}- Change worksapce.{color_reset}{style_reset}");

    // Author
    println!("{style_bold}{color_yellow}{}{color_reset}{style_reset}", AUTHOR);
    println!("Press return to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
