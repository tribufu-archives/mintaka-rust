// Copyright (c) Tribufu. All Rights Reserved.

use sea_query::Iden;

#[derive(Debug)]
pub struct StaticIden(pub &'static str);

impl Iden for StaticIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        s.write_str(self.0)
            .expect("StaticIden write_str fatal error");
    }
}
