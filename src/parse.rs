// MIT License
//
// Copyright (c) 2022 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod elite_scala {
    use elite::ast::EliteKeywords;

    fn replace(data: String) -> String {
        data.replace('\"', "\\'").replace("\0", "")
    }

    pub fn parse(data: elite::parser::EliteParser) -> String {
        let mut regenerated_code = String::from("\
import System.getProperty
import scala.language.postfixOps
import java.nio.file.{ Paths, Files }
import scala.sys.process._

object elite {
 def main(args: Array[String]) = {\n");
        let mut line = 2u32;
        let mut is_for = false;

        for x in data.ast_nodes.data {
            match x.__type {
                EliteKeywords::Set => {
                    regenerated_code.push_str(
                        format!("{}val {} = \"{}\"\n", " ".repeat(line as usize), x.__name, replace(x.__data)).as_str());
                }
                EliteKeywords::Print => {
                    regenerated_code.push_str(
                        format!("{}print(\"{}\")\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Println => {
                    regenerated_code.push_str(
                        format!("{}println(\"{}\\n\")\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Use => {}
                EliteKeywords::RequiredVersion => {
                    regenerated_code.push_str(format!("{}if(\"{}\" != \"{}\")\n{}{{\n{}{}}}\n",
                                                            " ".repeat(line as usize),
                                                            replace(x.__name),
                                                            replace(x.__data),
                                                            " ".repeat(line as usize),
                                                            "    println(\"elite: Required higher version\")\
                                                            \n    sys.exit(1);\n",
                                                            " ".repeat(line as usize)).as_str());
                }
                EliteKeywords::Change => {}
                EliteKeywords::IfArg => {
                    regenerated_code.push_str(format!("{}if(\"{}\"", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::LeftParenthese => {}
                EliteKeywords::RightParenthese => {}
                EliteKeywords::LeftSqBracket => {
                    regenerated_code.push_str(format!("{}{{\n", " ".repeat(line as usize)).as_str());
                    if is_for { is_for = false; continue; } line += 1;
                }
                EliteKeywords::RightSqBracket => {
                    regenerated_code.push_str(format!("{}}}\n", " ".repeat((line - 1) as usize)).as_str());
                    if line < 1 { continue } line -= 1;
                }
                EliteKeywords::Eq => {
                    regenerated_code.push_str(format!(" == \"{}\")\n", replace(x.__data)).as_str());
                }
                EliteKeywords::UnEq => {
                    regenerated_code.push_str(format!(" != \"{}\")\n", replace(x.__data)).as_str());
                }
                EliteKeywords::Signal => {
                    if x.__name == "exit" {
                        regenerated_code.push_str(format!("{}sys.exit(1)\n", " ".repeat(line as usize)).as_str());
                    } else if x.__name == "start" {
                        is_for = true;
                    }
                }
                EliteKeywords::Exec => {
                    regenerated_code.push_str(format!("{}Process(\"{}\")!\n", " ".repeat(line as usize), replace(x.__name)).as_str());
                }
                EliteKeywords::AddSource => {}
                EliteKeywords::Append => {}
                EliteKeywords::Exit => {
                    regenerated_code.push_str(format!("{}sys.exit(1)\n", " ".repeat(line as usize)).as_str());
                }
                EliteKeywords::Specific => {
                    match x.__data.as_str() {
                        "x86" => regenerated_code.push_str(
                            format!("{}if(getProperty(\"os.arch\") == \"i386\")\n", " ".repeat(line as usize)).as_str()),
                        "amd64" => regenerated_code.push_str(
                                format!("{}if(getProperty(\"os.arch\") == \"amd64\")\n", " ".repeat(line as usize)).as_str()),
                        "windows" => regenerated_code.push_str(
                            format!("{}if(getProperty(\"os.name\") == \"windows\")\n", " ".repeat(line as usize)).as_str()),
                        "macos" => regenerated_code.push_str(
                            format!("{}if(getProperty(\"os.name\") == \"macos\")\n", " ".repeat(line as usize)).as_str()),
                        "linux" => regenerated_code.push_str(
                            format!("{}if(getProperty(\"os.name\") == \"linux\")\n", " ".repeat(line as usize)).as_str()),
                        "freebsd" => regenerated_code.push_str(
                            format!("{}if(getProperty(\"os.name\") == \"freebsd\")\n", " ".repeat(line as usize)).as_str()),
                        "netbsd" => regenerated_code.push_str(
                            format!("{}if(getProperty(\"os.name\") == \"netbsd\")", " ".repeat(line as usize)).as_str()),
                        "android" => regenerated_code.push_str(
                            format!("{}if(getProperty(\"os.name\") == \"android\")\n", " ".repeat(line as usize)).as_str()),
                        _ =>
                            // other platforms are not directly supported but this is may be TODO.
                            regenerated_code.push_str(
                            format!("{} // not supported\n", " ".repeat(line as usize)).as_str())

                    }

                }
                EliteKeywords::Argument => {
                    regenerated_code.push_str(
                        format!("{}if(args.length >= 2 && args(1) == \"{}\")\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Exists => {
                    regenerated_code.push_str(
                        format!("{}val path = Files.exists(Paths.get(\"{}\"))\n\
                                       {}if(path)\n", " ".repeat(line as usize), replace(x.__data),
                                            " ".repeat(line as usize)).as_str());
                }
                EliteKeywords::Undefined => {},
                _ => {}
            }
        }

        regenerated_code.push_str(" }\n}");
        regenerated_code
    }
}