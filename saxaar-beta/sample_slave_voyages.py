import pandas as pd
import random
import argparse
from typing import List, Dict, Set

def sample_by_african_ports(input_csv: str, output_csv: str, n_samples: int) -> None:
    """
    Read slave voyage data and sample n rows for each African port.
    
    Args:
        input_csv: Path to input CSV file
        output_csv: Path to output CSV file
        n_samples: Number of rows to sample per port
    """
    # Read the CSV file
    print("Reading CSV file...")
    df = pd.read_csv(input_csv, low_memory=False)
    
    # Get African port columns
    port_cols = ['PLAC1TRA', 'PLAC2TRA', 'PLAC3TRA']
    
    # Dictionary to store voyages by port
    port_voyages: Dict[int, Set[int]] = {}
    
    # For each port column
    for col in port_cols:
        # Get rows with African ports (60000-69999)
        # First filter for the African port range
        mask = df[col].between(60000, 69999)
        if mask.any():
            african_ports = df[mask]
            # Get unique ports from this column
            unique_ports = african_ports[col].dropna().unique()
            
            # Process each unique port
            for port in unique_ports:
                # Get voyage IDs for this port
                voyage_ids = african_ports[african_ports[col] == port]['VOYAGEID'].tolist()
                if port not in port_voyages:
                    port_voyages[port] = set(voyage_ids)
                else:
                    port_voyages[port].update(voyage_ids)

    print(f"\nFound {len(port_voyages)} unique African ports")
    
    # Collect all sampled voyage IDs
    all_sampled_ids = set()
    
    # For each port, sample voyages
    for port, voyage_ids in sorted(port_voyages.items()):
        print(f"\nPort ID: {port}")
        print(f"Total voyages: {len(voyage_ids)}")
        
        # Sample voyages
        sample_size = min(n_samples, len(voyage_ids))
        sampled_ids = random.sample(list(voyage_ids), sample_size)
        all_sampled_ids.update(sampled_ids)
        
        print(f"Sampled {sample_size} voyages")

    # Get all sampled rows and save to CSV
    sampled_df = df[df['VOYAGEID'].isin(all_sampled_ids)]
    sampled_df.to_csv(output_csv, index=False)
    print(f"\nSaved {len(sampled_df)} sampled voyages to {output_csv}")

def main():
    parser = argparse.ArgumentParser(description='Sample slave voyage data by African ports')
    parser.add_argument('n_samples', type=int, help='Number of samples per port')
    parser.add_argument('output_file', type=str, help='Output CSV file path')
    args = parser.parse_args()
    
    input_csv = "fixtures/tastdb-exp-2019.csv"
    sample_by_african_ports(input_csv, args.output_file, args.n_samples)

if __name__ == "__main__":
    main()