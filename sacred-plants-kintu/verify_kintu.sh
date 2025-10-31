#!/bin/bash
set -e

H0="f6e0dfcf8258872c63c5f8416670de1f9e1d45c0bed7a78e01921c844104b0cf"
H1="5da539cb0863d4495c34c82df314b479b9a8cea0f7920ac242b15de5a0c7c95f"
H2="ace04661df207c84e01d611e8c71af4ad2ccc75b3d69ab88060dd7ef8ad9a7fe"
K="ae2c4c471e8122a051609f5fa9c86a2ef1b3583ff23aced18f002be8e58c8938"

R=("NC_030601.1" "HQ247200.1" "LC651165.1")

TMP=$(mktemp -d)
trap "rm -rf $TMP" EXIT

hash_fasta() {
    grep -v "^>" "$1" | tr -d '[:space:]' | tr '[:lower:]' '[:upper:]' | sha256sum | awk '{print $1}'
}

for i in 0 1 2; do
    curl -s "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=nucleotide&id=${R[$i]}&rettype=fasta&retmode=text" > "$TMP/$i.fasta"
    [ $i -eq 0 ] && head -100 "$TMP/$i.fasta" > "$TMP/$i.tmp" && mv "$TMP/$i.tmp" "$TMP/$i.fasta"
done

C0=$(hash_fasta "$TMP/0.fasta")
C1=$(hash_fasta "$TMP/1.fasta")
C2=$(hash_fasta "$TMP/2.fasta")

V=0
[ "$C0" == "$H0" ] && ((V++))
[ "$C1" == "$H1" ] && ((V++))
[ "$C2" == "$H2" ] && ((V++))

M01=$(echo -n "${C0}${C1}" | sha256sum | awk '{print $1}')
M22=$(echo -n "${C2}${C2}" | sha256sum | awk '{print $1}')
MK=$(echo -n "${M01}${M22}" | sha256sum | awk '{print $1}')

[ "$MK" == "$K" ] && ((V++))

[ $V -eq 4 ] && exit 0 || exit 1
