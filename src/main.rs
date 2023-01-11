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
            if !name.starts_with(publisher) {
                continue;
            }

            let mut new_name = name.replace(publisher, "");
            let index = new_name.rfind(".").unwrap_or_else(|| {
                panic!("Can not find the right most dot")
            });
            new_name.insert_str(index + 1, publisher);

            let parent = entry.path().parent().unwrap_or_else(|| {
                panic!("Get parent directory error")
            });
            fs::rename(entry.path(), parent.join(new_name)).unwrap_or_else(|error| {
                panic!("Problem renaming the file: {:?}", error);
            });
            break;
        }
    }
}
