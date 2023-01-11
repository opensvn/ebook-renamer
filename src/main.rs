use std::fs;
use walkdir::WalkDir;

fn main() {
    let publishers = vec![
        "Apress.",
        "Arcler.Press.",
        "AW.",
        "BCS.",
        "Cambridge.University.Press.",
        "Cengage.",
        "Course.Technology.",
        "CRC.",
        "CreateSpace.",
        "DK.",
        "For.Dummies.",
        "Knopf.",
        "IET.",
        "MGH.",
        "Manning.",
        "Microsoft.Press.Code.",
        "No.Starch.Press.",
        "NYU.Press.",
        "Open.University.Press.",
        "OReilly.",
        "Oxford.University.Press.",
        "Packt.",
        "Pearson.",
        "Pragmatic.Bookshelf.",
        "Prentice.Hall.",
        "Routledge.",
        "SAGE.",
        "Sams.Publishing.",
        "SitePoint.",
        "Springer.",
        "The.MIT.Press.",
        "Wiley.",
        "Williams.Publishing.",
        "WSC.",
    ];

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {
        let name = String::from(entry.file_name().to_string_lossy());
        for publisher in &publishers {
            if name.clone().starts_with(publisher) {
                let mut new_name = name.replace(publisher, "");
                let index = new_name.rfind(".");
                match index {
                    Some(i) => {
                        new_name.insert_str(i + 1, publisher);
                        match entry.path().parent() {
                            Some(p) => {
                                let result = fs::rename(entry.path(), p.join(new_name));
                                match result {
                                    Err(e) => println!("{}", e),
                                    _ => {}
                                }
                                break;
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
        }
    }
}
