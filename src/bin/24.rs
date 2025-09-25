use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug)]
enum Operation {
    AND,
    XOR,
    OR,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::AND,
            "XOR" => Self::XOR,
            "OR" => Self::OR,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Gate {
    inputs: Vec<usize>,
    operation: Operation,
    output: usize,
}

impl Gate {
    fn is_already_computed(&self, wire_values: &HashMap<usize, usize>) -> bool {
        wire_values.contains_key(&self.output)
    }

    fn is_ready_to_compute(&self, wire_values: &HashMap<usize, usize>) -> bool {
        self.inputs.iter().all(|i| wire_values.contains_key(i))
    }

    fn compute(&mut self, wire_values: &HashMap<usize, usize>) -> usize {
        let mut result = match self.operation {
            Operation::AND => usize::MAX,
            Operation::OR | Operation::XOR => usize::MIN,
        };

        for i in self.inputs.iter() {
            let value = wire_values.get(&i).unwrap();
            match self.operation {
                Operation::AND => result &= value,
                Operation::XOR => result ^= value,
                Operation::OR => result |= value,
            }
        }

        // println!(
        //     "Compute {:?}: {:?} -> {:?} = {:?}",
        //     self.operation, self.inputs, self.output, result
        // );

        result & 0b1
    }
}

#[derive(Debug, Default)]
struct Device {
    wire_names: Vec<String>,
    wire_values: HashMap<usize, usize>,
    gates: Vec<Gate>,
}

impl Device {
    fn new(input: &str) -> Self {
        let mut device: Device = Default::default();
        let (initial_settings_string, gates_string) = input.trim().split_once("\n\n").unwrap();

        for gate_line in gates_string.lines() {
            let (input_op, output_text) = gate_line.trim().split_once(" -> ").unwrap();
            let input_op_split = input_op.split_whitespace().collect::<Vec<_>>();
            device.add_gate(
                input_op_split[0].to_string(),
                input_op_split[2].to_string(),
                input_op_split[1].to_string(),
                output_text.to_string(),
            );
        }

        for wire_setting in initial_settings_string.lines() {
            let (wire_name_str, value_str) = wire_setting.split_once(": ").unwrap();
            let wire_idx = device
                .wire_names
                .iter()
                .position(|w| *w == wire_name_str)
                .unwrap();
            device
                .wire_values
                .insert(wire_idx, value_str.parse::<usize>().unwrap());
        }

        // println!("{:?}", device.wire_names);

        device
    }

    fn add_gate(
        &mut self,
        input1_str: String,
        input2_str: String,
        op_str: String,
        output_str: String,
    ) {
        let input1_idx = if self.wire_names.contains(&input1_str) {
            self.wire_names
                .iter()
                .position(|n| *n == input1_str)
                .unwrap()
        } else {
            self.wire_names.push(input1_str);
            self.wire_names.len() - 1
        };
        let input2_idx = if self.wire_names.contains(&input2_str) {
            self.wire_names
                .iter()
                .position(|n| *n == input2_str)
                .unwrap()
        } else {
            self.wire_names.push(input2_str);
            self.wire_names.len() - 1
        };
        let output_idx = if self.wire_names.contains(&output_str) {
            self.wire_names
                .iter()
                .position(|n| *n == output_str)
                .unwrap()
        } else {
            self.wire_names.push(output_str);
            self.wire_names.len() - 1
        };
        let operation = Operation::from(op_str.as_str());
        self.gates.push(Gate {
            inputs: vec![input1_idx, input2_idx],
            operation: operation,
            output: output_idx,
        });
    }

    fn get_label(&self, index: usize) -> &str {
        self.wire_names.get(index).unwrap()
    }

    fn step(&mut self) -> bool {
        for gate in self.gates.iter_mut() {
            if !gate.is_already_computed(&self.wire_values)
                && gate.is_ready_to_compute(&self.wire_values)
            {
                self.wire_values
                    .insert(gate.output, gate.compute(&self.wire_values));
                return true;
            }
        }
        false
    }

    fn z_output(&self) -> usize {
        let z_idxs = self
            .wire_names
            .iter()
            .filter(|w| w.starts_with("z"))
            .sorted()
            .collect::<Vec<_>>();

        let mut result = 0;
        for (idx, z) in z_idxs.iter().enumerate() {
            let wire_idx = self.wire_names.iter().position(|w| w == *z).unwrap();
            let value = *self.wire_values.get(&wire_idx).unwrap();
            // println!("{:02} | {} | {}", idx, z, value);
            result |= value << idx;
        }
        result
    }

    fn reset(&mut self) {
        self.wire_values = HashMap::new();
    }

    fn set_input(&mut self, wire_prefix: &str, input: usize) {
        let idxs = self
            .wire_names
            .iter()
            .filter(|w| w.starts_with(wire_prefix))
            .sorted()
            .collect::<Vec<_>>();

        for (idx, wn) in idxs.iter().enumerate() {
            let wire_idx = self.wire_names.iter().position(|w| w == *wn).unwrap();
            self.wire_values.insert(wire_idx, (input >> idx) & 0b1);
        }
        // println!("{:?}", self.wire_names);
        // println!("{:?}", self.wire_values);
    }

    fn trace_paths(&self, input: usize, output: usize, current_path: &[usize]) -> Vec<Vec<usize>> {
        let mut paths = vec![];

        for gate in self.gates.iter().filter(|g| g.inputs.contains(&input)) {
            if gate.output == output {
                let path = current_path
                    .iter()
                    .cloned()
                    .chain([output])
                    .collect::<Vec<_>>();
                paths.push(path);
                continue;
            }

            let new_current_path = current_path
                .iter()
                .cloned()
                .chain([gate.output])
                .collect::<Vec<_>>();
            paths.extend(self.trace_paths(gate.output, output, &new_current_path));
        }

        paths
    }

    fn find_wrong_input_output_pairs(&mut self, prefix: &str) -> Vec<(usize, usize)> {
        let mut pairs = vec![];
        for i in 0..45 {
            let value = 0b1 << i;
            self.reset();
            self.set_input("x", 0b0);
            self.set_input("y", 0b0);
            self.set_input(prefix, value);
            while self.step() {}
            let result = self.z_output();
            if value != result {
                let error1_offset = value.trailing_zeros();
                let error1_x = self
                    .wire_names
                    .iter()
                    .position(|w| *w == format!("{}{:02}", prefix, error1_offset))
                    .unwrap();
                let error1_z = self
                    .wire_names
                    .iter()
                    .position(|w| *w == format!("z{:02}", error1_offset))
                    .unwrap();
                pairs.push((error1_x, error1_z));
                let error2_offset = result.trailing_zeros();
                let error2_x = self
                    .wire_names
                    .iter()
                    .position(|w| *w == format!("{}{:02}", prefix, error2_offset))
                    .unwrap();
                let error2_z = self
                    .wire_names
                    .iter()
                    .position(|w| *w == format!("z{:02}", error2_offset))
                    .unwrap();
                pairs.push((error2_x, error2_z));
            }
        }
        pairs
    }

    fn output_dot_graph(&self) -> String {
        let mut result = String::new();

        for (idx, gate) in self.gates.iter().enumerate() {
            let gate_label = match gate.operation {
                Operation::AND => format!("A{}", idx),
                Operation::XOR => format!("X{}", idx),
                Operation::OR => format!("O{}", idx),
            };
            result.push_str(&format!(
                "{} -> {}\n",
                gate_label,
                self.get_label(gate.output)
            ));
            for i in &gate.inputs {
                result.push_str(&format!("{} -> {}\n", self.get_label(*i), gate_label));
            }
        }

        result
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut device = Device::new(input);
    while device.step() {
        // println!("{:?}", device.wire_values);
    }
    Some(device.z_output())
}

pub fn part_two(_input: &str) -> Option<String> {
    // We look by hand. First identify possible suspicious lines and then look at a visualization to see the real problems.
    // let mut device = Device::new(input);
    // println!("Nodes: {:?}", device.wire_names.iter().sorted());
    // let suspicious_pairs = device.find_wrong_input_output_pairs("x");
    // for (input, output) in suspicious_pairs {
    //     println!(
    //         "Found {} and {}.",
    //         device.get_label(input),
    //         device.get_label(output)
    //     );
    //     println!("Traces:");
    //     for trace in device.trace_paths(input, output, &vec![input]) {
    //         println!("  - {:?}", trace);
    //     }
    //     println!("----------------------");
    // }
    // let suspicious_pairs = device.find_wrong_input_output_pairs("y");
    // for (input, output) in suspicious_pairs {
    //     println!(
    //         "Found {} and {}.",
    //         device.get_label(input),
    //         device.get_label(output)
    //     );
    //     println!("Traces:");
    //     for trace in device.trace_paths(input, output, &vec![input]) {
    //         println!("  - {:?}", trace);
    //     }
    //     println!("----------------------");
    // }
    // println!("{}", device.output_dot_graph());

    // Graphviz visualization shows that:
    // 1.   x07 AND y07 -> z07
    //      mvw XOR pmc -> gmt
    // 2.   stg OR  khk -> z18
    //      nff XOR hch -> dmn
    // 3.   qnm AND rfk -> z35
    //      qnm XOR rfk -> cfk
    // 4.   x11 XOR y11 -> qjj
    //      x11 AND y11 -> cbj
    // need to have their output swapped
    Some("cbj,cfk,dmn,gmt,qjj,z07,z18,z35".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }
}
