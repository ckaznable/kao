use std::{borrow::Cow, sync::LazyLock};

pub const ORI_SVG_WIDTH: u32 = 200;
pub const ORI_SVG_HEIGHT: u32 = 120;

pub static HAPPY_FACE: LazyLock<Cow<'static, str>> = LazyLock::new(|| {
    Cow::Borrowed(FaceBuilder::default()
        .mouth(Mouth::Angry)
        .eye_brows(EyeBrows::Up)
        .build()
        .leak())
});

pub static NORMAL_FACE: LazyLock<Cow<'static, str>> = LazyLock::new(|| {
    Cow::Borrowed(FaceBuilder::default().build().leak())
});

pub static ANGRY_FACE: LazyLock<Cow<'static, str>> = LazyLock::new(|| {
    Cow::Borrowed(FaceBuilder::default()
        .mouth(Mouth::Angry)
        .eye_brows(EyeBrows::Up)
        .build()
        .leak())
});

#[derive(Copy, Clone, Debug, Default)]
pub enum Face {
    #[default]
    Normal,
    Happy,
    Angry,
}

impl Face {
    fn to_svg(self) -> Cow<'static, str> {
        match self {
            Face::Normal => NORMAL_FACE.clone(),
            Face::Happy => HAPPY_FACE.clone(),
            Face::Angry => ANGRY_FACE.clone(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
enum Pupils {
    #[default]
    Default,
    None,
}

#[derive(Copy, Clone, Debug)]
enum Eye {
    Open(Pupils, Pupils),
    Close,
}

impl Default for Eye {
    fn default() -> Self {
        Self::Open(Pupils::default(), Pupils::default())
    }
}

#[derive(Copy, Clone, Debug, Default)]
enum Mouth {
    #[default]
    W,
    O,
    Smile,
    Angry,
}

#[derive(Copy, Clone, Debug, Default)]
enum EyeBrows {
    Up,
    Down,
    #[default]
    Flat,
}

#[derive(Copy, Clone, Debug, Default)]
struct FaceBuilder {
    eye: Eye,
    mouth: Mouth,
    eye_brows: EyeBrows,
}

impl FaceBuilder {
    fn eye(mut self, e: Eye) -> Self {
        self.eye = e;
        self
    }

    fn mouth(mut self, m: Mouth) -> Self {
        self.mouth = m;
        self
    }

    fn eye_brows(mut self, e: EyeBrows) -> Self {
        self.eye_brows = e;
        self
    }

    fn build(self) -> String {
        const SVG_START: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 120">"##;
        const SVG_END: &str = "</svg>";

        let eye = match self.eye {
            Eye::Open(l, r) => {
                &format!(
                    "{}{}",
                    if matches!(l, Pupils::Default) {
                        r##"<circle cx="65" cy="60" r="8"/>"##
                    } else {
                        ""
                    },
                    if matches!(r, Pupils::Default) {
                        r##"<circle cx="135" cy="60" r="8"/>"##
                    } else {
                        ""
                    },
                )
            },
            Eye::Close => r##"
                <line x1="45" y1="60" x2="85" y2="60" stroke-width="3"/>
                <line x1="115" y1="60" x2="155" y2="60" stroke-width="3"/>
            "##,
        };

        let eye_brows = match self.eye_brows {
            EyeBrows::Up => r##"
                <line x1="50" y1="35" x2="75" y2="42" stroke-width="3"/>
                <line x1="150" y1="35" x2="125" y2="42" stroke-width="3"/>
            "##,
            EyeBrows::Down => r##"
                <line x1="70" y1="35" x2="50" y2="42" stroke-width="3"/>
                <line x1="125" y1="35" x2="150" y2="42" stroke-width="3"/>
            "##,
            EyeBrows::Flat => r##"
                <line x1="55" y1="35" x2="75" y2="35" stroke="#1E90FF" stroke-width="3"/>
                <line x1="125" y1="35" x2="145" y2="35" stroke="#1E90FF" stroke-width="3"/>
            "##,
        };

        let mouth = match self.mouth {
            Mouth::W => r##"<path d="M135 130 Q145 140 150 130 Q155 140 165 130" stroke-width="3" fill="none"/>"##,
            Mouth::O => r##"<circle cx="100" cy="95" r="6" stroke-width="2" fill="none"/>"##,
            Mouth::Smile => r##"<path d="M90 85 Q100 95 110 85" stroke-width="2" fill="none"/>"##,
            Mouth::Angry => r##"<path d="M80 95 Q100 85 120 95" stroke-width="2" fill="none"/>"##,
        };

        const NOSE: &str = r##"<circle cx="100" cy="80" r="3"/>"##;
        const WHISKERS: &str = r##"
            <line x1="30" y1="70" x2="45" y2="73" stroke-width="2"/>
            <line x1="30" y1="80" x2="45" y2="80" stroke-width="2"/>
            <line x1="30" y1="90" x2="45" y2="87" stroke-width="2"/>
            <line x1="170" y1="70" x2="155" y2="73" stroke-width="2"/>
            <line x1="170" y1="80" x2="155" y2="80" stroke-width="2"/>
            <line x1="170" y1="90" x2="155" y2="87" stroke-width="2"/>
        "##;

        let svg = [SVG_START, eye, eye_brows, mouth, NOSE, WHISKERS, SVG_END];
        let mut buf = String::with_capacity(svg.map(|s| s.len()).iter().sum());
        svg.iter().for_each(|s| buf.push_str(s));
        buf
    }
}

