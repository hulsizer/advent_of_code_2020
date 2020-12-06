

struct Passport {
    cid: Option<std::string::String>,
    byr: Option<std::string::String>,
    iyr: Option<std::string::String>,
    eyr: Option<std::string::String>,
    hgt: Option<std::string::String>,
    hcl: Option<std::string::String>,
    ecl: Option<std::string::String>,
    pid: Option<std::string::String>,
}

impl std::fmt::Display for Passport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "cid: {:?}, byr: {:?}, iyr: {:?}, eyr: {:?}, hgt: {:?}, hcl: {:?}, ecl: {:?}, pid: {:?}", self.cid, self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid);
    }
}

impl Passport {

    fn new(values: &str) -> Passport {

        let mut passport = Passport {
            cid: None, 
            byr: None,
            iyr: None,
            eyr: None, 
            hgt: None, 
            hcl: None,
            ecl: None, 
            pid: None,
        };
        values.replace("\n", " ").split(" ")
            .for_each(|field| {
                let mut iter = field.split(":");
                let name = iter.next().unwrap();
                let value = iter.next().unwrap();

                match name {
                    "cid" => passport.cid = Some(value.to_string()),
                    "byr" => passport.byr = Some(value.to_string()),
                    "iyr" => passport.iyr = Some(value.to_string()),
                    "eyr" => passport.eyr = Some(value.to_string()),
                    "hgt" => passport.hgt = Some(value.to_string()),
                    "hcl" => passport.hcl = Some(value.to_string()),
                    "ecl" => passport.ecl = Some(value.to_string()),
                    "pid" => passport.pid = Some(value.to_string()),
                    _ => ()
                }
            });

        return passport;
    }

    //Part 1
    // fn is_valid(&self) -> bool {
    //     return self.byr.is_some() &&
    //            self.iyr.is_some() &&
    //            self.hgt.is_some() && 
    //            self.iyr.is_some() &&
    //            self.eyr.is_some() &&
    //            self.hcl.is_some() &&
    //            self.ecl.is_some() &&
    //            self.pid.is_some()
    // }
    
    //Part 2
    fn is_valid(&self) -> bool {

        //Birth
        if self.byr.is_none() || !("1920" <= self.byr.as_ref().unwrap() && "2002" >= self.byr.as_ref().unwrap()) {
            return false
        }

        //
        if self.iyr.is_none() || !("2010" <= self.iyr.as_ref().unwrap() && "2020" >= self.iyr.as_ref().unwrap()) {
            return false
        }

        //Expiration
        if self.eyr.is_none() || !("2020" <= self.eyr.as_ref().unwrap() && "2030" >= self.eyr.as_ref().unwrap()) {
            return false
        }

        //Height
        if self.hgt.is_none() {
            return false
        }

        if self.hgt.as_ref().unwrap().contains("cm") {
            let number_str = self.hgt.as_ref().unwrap()[..self.hgt.as_ref().unwrap().len() - 2].to_string();
            if !("150".to_string() <= number_str && "193".to_string() >= number_str) {
                return false
            }
        } else if self.hgt.as_ref().unwrap().contains("in") {
            let number_str = self.hgt.as_ref().unwrap()[..self.hgt.as_ref().unwrap().len() - 2].to_string();
            if !("59".to_string() <= number_str && "76".to_string() >= number_str) {
                return false
            }
        } else {
            println!("Height {:?}", self.hgt);
            return false
        }

        //Hair Color
        if self.hcl.is_none() {
            return false
        }

        if self.hcl.as_ref().unwrap()[..1].to_string() != "#" {
            return false
        }

        let result = self.hcl.as_ref().unwrap()[1..].chars().filter( |char| {
            match char {
                '0'..='9' => return true,
                'a'..='z' => return true,
                _ => return false
            }
        });

        if result.count() != 6 {
            return false
        }
        
        //Eye Color
        if self.ecl.is_none() {
            return false
        }

        match self.ecl.as_ref().unwrap().as_str() {
            "amb" => (),
            "blu" => (),
            "brn" => (),
            "gry" => (),
            "grn" => (),
            "hzl" => (),
            "oth" => (),
            _ => {
                return false
            }
        }

        //Passport Id
        if self.pid.is_none() {
            return false
        }

        if self.pid.as_ref().unwrap().len() != 9 {
            return false
        }

        if self.pid.as_ref().unwrap().parse::<i32>().is_err() {
            return false
        }

        //

        return true
    }
}
pub fn run(input: std::string::String) {
    let valid_passports: Vec<Passport> = input.split("\n\n")
        .map( | passport | Passport::new(passport))
        .filter(|passport| passport.is_valid())
        .collect();

    println!("{}", valid_passports.len());
}