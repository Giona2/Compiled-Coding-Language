use colored::Color;

pub trait ColorEq {
    fn to_string(&self) -> String;
}

impl ColorEq for Color {
    fn to_string(&self) -> String { match self {
        Self::Red     => { "Red".into()     }
        Self::Blue    => { "Blue".into()    }
        Self::Cyan    => { "Cyan".into()    }
        Self::Black   => { "Black".into()   }
        Self::White   => { "White".into()   }
        Self::Yellow  => { "Yellow".into()  }
        Self::Magenta => { "Magenta".into() }
        Self::Green   => { "Green".into()   }
                    _ => { "".into()        }
    }}
}
