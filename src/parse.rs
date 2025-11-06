use crate::structure::{Atom, Crystal};
use anyhow::{Context, Result};
use std::collections::HashMap;

// Function to parse XYZ file format from string content
pub fn parse_xyz_content(contents: &str) -> Result<Crystal> {
    let lines = contents.lines().collect::<Vec<&str>>();

    if lines.len() < 2 {
        return Err(anyhow::anyhow!("XYZ file too short"));
    }

    // First line should contain the number of atoms
    let num_atoms: usize = lines[0]
        .trim()
        .parse()
        .context("Failed to parse number of atoms")?;

    // Second line may contain comment or extended XYZ properties
    let comment_line = lines[1].trim();
    
    // Parse extended XYZ properties if present
    let mut properties = HashMap::new();
    if comment_line.starts_with("Lattice=\"") || comment_line.contains("Properties=") {
        parse_extended_xyz_properties(comment_line, &mut properties);
    }

    let mut atoms = Vec::new();

    for (i, line) in lines.iter().skip(2).enumerate() {
        if i >= num_atoms {
            break;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue; // Skip malformed lines
        }

        let atom = Atom {
            element: parts[0].to_string(),
            x: parts[1].parse().context("Failed to parse x coordinate")?,
            y: parts[2].parse().context("Failed to parse y coordinate")?,
            z: parts[3].parse().context("Failed to parse z coordinate")?,
        };

        atoms.push(atom);
    }

    Ok(Crystal { atoms })
}

// Parse extended XYZ properties (basic implementation)
fn parse_extended_xyz_properties(comment: &str, _properties: &mut HashMap<String, String>) {
    // This is a simplified parser for extended XYZ format
    // For full implementation, refer to: https://github.com/libAtoms/extxyz
    if comment.starts_with("Lattice=\"") {
        // Extract lattice parameters if needed
    }
    // Add more property parsing as needed
}

// Function to read XYZ file from path
#[allow(dead_code)]
pub fn read_xyz_file(path: &str) -> Result<Crystal> {
    let contents = std::fs::read_to_string(path)
        .context(format!("Failed to read XYZ file: {}", path))?;
    parse_xyz_content(&contents)
}
