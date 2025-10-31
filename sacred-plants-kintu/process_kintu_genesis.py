#!/usr/bin/env python3
"""
ANDE Chain - K'intu Sacred Plants Genesis Integration
Integración Profesional de Datos Genómicos en Blockchain

Basado en best practices de:
- Genome Biology (2022): "Storing and analyzing a genome on a blockchain"
- PMC 10209554: "A review on blockchain for DNA sequence"
- SHA-256 hashing + Merkle Tree verification
"""

import hashlib
import json
from pathlib import Path
from typing import Dict, List, Tuple
import binascii

class KintuGenomicProcessor:
    """Procesa secuencias genómicas de Plantas Sagradas para genesis blockchain"""

    PLANTS = {
        "kuka": {
            "name_indigenous": "Kuka",
            "name_spiritual": "Mamacoca",
            "name_scientific": "Erythroxylum novogranatense",
            "file": "kuka_erythroxylum_chloroplast_NC030601.fasta",
            "ncbi_accession": "NC_030601.1",
            "gene": "chloroplast_genome",
            "cultural_meaning": "La raíz, la tierra, el nodo estable de la triada K'intu. Representa la conexión con Mama Pachamama.",
            "etymology": "Del quechua 'kuka' y aymara 'kkoka' - planta sagrada ancestral"
        },
        "yage": {
            "name_indigenous": "Yagé",
            "name_spiritual": "Ayahuasca",
            "name_scientific": "Banisteriopsis caapi",
            "file": "yage_banisteriopsis_caapi_matK_CORRECTED.fasta",
            "ncbi_accession": "HQ247200.1",
            "gene": "chloroplast_matK",
            "cultural_meaning": "La liana, el espíritu. 'La soga de los espíritus' que permite la visión sagrada.",
            "etymology": "Del cofán 'yagé' y quechua 'aya' (espíritu) + 'waska' (soga)"
        },
        "chacruna": {
            "name_indigenous": "Chacruna",
            "name_spiritual": "Chaqruy",
            "name_scientific": "Psychotria viridis",
            "file": "chacruna_psychotria_viridis_rbcL_CORRECTED.fasta",
            "ncbi_accession": "LC651165.1",
            "gene": "chloroplast_rbcL",
            "cultural_meaning": "La visión, la mezcla, el catalizador del conocimiento. Sin ella, la liana está muda.",
            "etymology": "Del quechua 'chaqruy' (mezclar) - la hoja de la visión"
        }
    }

    def __init__(self, data_dir: str):
        self.data_dir = Path(data_dir)
        self.sequences = {}

    def parse_fasta(self, filepath: Path) -> Tuple[str, str]:
        """Parse FASTA file and return header and sequence"""
        with open(filepath, 'r') as f:
            lines = f.readlines()

        header = ""
        sequence = ""

        for line in lines:
            line = line.strip()
            if line.startswith('>'):
                header = line[1:]  # Remove '>'
            elif line:
                sequence += line.upper()

        return header, sequence

    def calculate_sha256(self, sequence: str) -> str:
        """Calculate SHA-256 hash of DNA sequence"""
        return hashlib.sha256(sequence.encode()).hexdigest()

    def compress_sequence_simple(self, sequence: str) -> str:
        """
        Simple compression for blockchain storage
        Encode DNA as 2-bit per base: A=00, C=01, G=10, T=11
        """
        encoding = {'A': '00', 'C': '01', 'G': '10', 'T': '11', 'N': '00'}

        # Convert to binary string
        binary = ''.join(encoding.get(base, '00') for base in sequence)

        # Pad to multiple of 8
        padding = (8 - len(binary) % 8) % 8
        binary += '0' * padding

        # Convert to hex
        hex_compressed = hex(int(binary, 2))[2:]

        return hex_compressed

    def merkle_root(self, hashes: List[str]) -> str:
        """Calculate Merkle root from list of hashes"""
        if len(hashes) == 1:
            return hashes[0]

        if len(hashes) % 2 == 1:
            hashes.append(hashes[-1])  # Duplicate last if odd

        new_level = []
        for i in range(0, len(hashes), 2):
            combined = hashes[i] + hashes[i + 1]
            new_hash = hashlib.sha256(combined.encode()).hexdigest()
            new_level.append(new_hash)

        return self.merkle_root(new_level)

    def process_plant(self, plant_key: str) -> Dict:
        """Process single plant genomic data"""
        plant_info = self.PLANTS[plant_key]
        filepath = self.data_dir / plant_info["file"]

        print(f"📊 Procesando {plant_info['name_indigenous']} ({plant_info['name_scientific']})...")

        header, sequence = self.parse_fasta(filepath)

        # Calculate hashes
        original_hash = self.calculate_sha256(sequence)

        # Simple compression (2-bit encoding)
        compressed = self.compress_sequence_simple(sequence)

        # Calculate compression ratio
        original_size = len(sequence)
        compressed_size = len(compressed) // 2  # hex chars to bytes
        compression_ratio = ((original_size - compressed_size) / original_size) * 100

        print(f"  ✅ Secuencia: {original_size} bp")
        print(f"  ✅ Hash SHA-256: {original_hash[:16]}...")
        print(f"  ✅ Comprimida: {compressed_size} bytes ({compression_ratio:.1f}% reducción)")

        return {
            "name_indigenous": plant_info["name_indigenous"],
            "name_spiritual": plant_info["name_spiritual"],
            "name_scientific": plant_info["name_scientific"],
            "gene": plant_info["gene"],
            "ncbi_accession": plant_info["ncbi_accession"],
            "ncbi_verification_url": f"https://www.ncbi.nlm.nih.gov/nucleotide/{plant_info['ncbi_accession']}",
            "sequence_length_bp": original_size,
            "sha256_hash": original_hash,
            "compressed_hex": compressed,
            "compressed_size_bytes": compressed_size,
            "compression_ratio_percent": round(compression_ratio, 2),
            "cultural_meaning": plant_info["cultural_meaning"],
            "etymology": plant_info["etymology"],
            "fasta_header": header
        }

    def generate_genesis_data(self) -> Dict:
        """Generate complete genesis data for K'intu"""
        print("\n🌿 Procesando Plantas Sagradas K'intu para ANDE Chain Genesis\n")

        # Process all three plants
        kuka_data = self.process_plant("kuka")
        yage_data = self.process_plant("yage")
        chacruna_data = self.process_plant("chacruna")

        # Calculate Merkle root
        all_hashes = [
            kuka_data["sha256_hash"],
            yage_data["sha256_hash"],
            chacruna_data["sha256_hash"]
        ]
        merkle_root = self.merkle_root(all_hashes)

        print(f"\n🔐 Merkle Root K'intu: {merkle_root[:32]}...")

        # Create genesis structure
        genesis_kintu = {
            "kintu": {
                "description": "Plantas Sagradas K'intu - Conocimiento Ancestral Verificable",
                "version": "1.0.0",
                "methodology": "SHA-256 + Merkle Tree + NCBI Verification",
                "timestamp": "2025-10-29T00:00:00Z",
                "merkle_root": merkle_root,
                "plants": {
                    "kuka": kuka_data,
                    "yage": yage_data,
                    "chacruna": chacruna_data
                },
                "verification": {
                    "method": "NCBI GenBank Cross-Reference",
                    "instructions": "Download sequences from NCBI using accession numbers, calculate SHA-256, compare with on-chain hashes",
                    "ncbi_database": "https://www.ncbi.nlm.nih.gov/nucleotide/",
                    "verification_script": "verify_kintu.sh"
                },
                "cultural_heritage": {
                    "tradition": "K'intu - Triada Sagrada de los Pueblos Originarios",
                    "languages": ["Quechua", "Aymara", "Cofán"],
                    "significance": "Preservación del conocimiento ancestral con verificación científica",
                    "declaration": "Conocimiento preservado en las lenguas originarias de los pueblos que la conocen desde hace milenios"
                }
            }
        }

        return genesis_kintu

def main():
    processor = KintuGenomicProcessor("/mnt/c/Users/sator/andelabs/ande-reth/sacred-plants-kintu")

    genesis_data = processor.generate_genesis_data()

    # Save to JSON
    output_file = Path("/mnt/c/Users/sator/andelabs/ande-reth/sacred-plants-kintu/kintu_genesis_data.json")
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(genesis_data, f, indent=2, ensure_ascii=False)

    print(f"\n✅ Datos de genesis guardados en: {output_file}")
    print("\n📋 Resumen:")
    print(f"  • 3 Plantas Sagradas procesadas")
    print(f"  • Merkle Root: {genesis_data['kintu']['merkle_root'][:32]}...")
    print(f"  • Total secuencias: {sum(p['sequence_length_bp'] for p in genesis_data['kintu']['plants'].values())} bp")
    print(f"  • Verificable contra NCBI GenBank")
    print(f"\n🌿 K'intu completo - Listo para integrar en genesis.json")

if __name__ == "__main__":
    main()
