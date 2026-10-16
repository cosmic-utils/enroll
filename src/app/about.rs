use cosmic::widget::{about::About, icon};

use crate::fl;

pub fn about(icon_name: &str) -> About {
    About::default()
        .name(fl!("app-title"))
        .icon(icon::from_name(icon_name))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joonas Tuomi")
        .translators([
            ("Luna \"bittin\" Jernberg", "bittin@reimu.nl"),
            ("David Carvalho", "david.snt.carvalho@gmail.com"),
            ("Geeson \"Gerseneck\" Wan", "wang14240@gmail.com"),
            ("Albano Battistella", ""),
            ("Pablo García \"PGSCOM\" Santamaría", ""),
            ("lorduskordus", "lorduskordus@gmail.com"),
            ("therealmate", "hellogaming91@gmail.com"),
            ("VandaLHJ", "vandalhj@gmail.com"),
            ("Димко", "Dymkovych@proton.me"),
        ])
        .designers([("KodeBarista", ""), ("Hojjat Abdollahi", "")])
        .comments(fl!("welcome"))
        .license(env!("CARGO_PKG_LICENSE"))
        .license_url("https://spdx.org/licenses/MPL-2.0")
        .developers([("Joonas Tuomi", "github@joonastuomi.fi")])
        .links([
            (fl!("repository"), "https://github.com/cosmic-utils/enroll"),
            (
                fl!("support"),
                "https://github.com/cosmic-utils/enroll/issues",
            ),
        ])
}
