import os
import sys


def main():
    try:
        day_number = int(sys.argv[1])
    except IndexError:
        print("No day number passed. Pass one with: `python3 day_setup.py <day-number>")
        return
    except ValueError:
        print("Could not convert argument to day number")
        return

    target_directory = f"src/day{day_number}/"
    if os.path.exists(target_directory):
        should_continue = input(f"Warning: directory for day {day_number} "
                                f"already exists. Proceeding will delete the directory's contents. Proceed? [y/N] ")\
            .lower()
        if should_continue != "y":
            print("Aborting...")
    else:
        os.mkdir(target_directory)

    with open(target_directory + "mod.rs", "w") as f:
        f.write(f"""use crate::day::{{Answer, Day}};

pub struct Day{day_number};
impl Day for Day{day_number} {{
    type TypePart1 = u32;
    type TypePart2 = u32;

    fn run(&self) -> Answer<Self::TypePart1, Self::TypePart2> {{
        let input = self.get_input_for_day_by_line(5);

        Answer::new(None, None)
    }}
}}""")

    f = open(target_directory + "input.txt", "w")
    f.close()

    with open("src/lib.rs", "a") as f:
        f.write(f"pub mod day{day_number};\n")


if __name__ == "__main__":
    main()
