"""Check that every dbp: term used in README.md is defined in dbp-schema.jsonld.

The README documents the vocabulary by example, so a term written there with the
wrong spelling — most often the wrong casing, such as `dbp:dbpaCompressColumnID`
for `dbp:dbpaCompressColumnId` — reads as a real property but resolves to
nothing. Nothing in the build catches that: the .jsonld is generated from the
.proto by ProtoToJsonld.py and never looks at the README.

Run from the repository root; no third-party dependency:

    python3 check_readme_terms.py

Exits non-zero and lists the offending terms when the README uses one the
vocabulary does not define.
"""

import json
import re
import sys

README = 'README.md'
VOCABULARY = 'dbp-schema.jsonld'

TERM_PATTERN = re.compile(r'\bdbp:[A-Za-z_][A-Za-z0-9_]*')


def defined_terms(jsonldfile):
    with open(jsonldfile, encoding='UTF-8') as f:
        return {node['@id'] for node in json.load(f)['@graph']}


def used_terms(mdfile):
    with open(mdfile, encoding='UTF-8') as f:
        return sorted(set(TERM_PATTERN.findall(f.read())))


def main():
    defined = defined_terms(VOCABULARY)
    undefined = [term for term in used_terms(README) if term not in defined]
    if undefined:
        print(
            f'{README} uses dbp: terms that {VOCABULARY} does not define:',
            file=sys.stderr,
        )
        for term in undefined:
            print(f'  {term}', file=sys.stderr)
        return 1
    print(f'{README}: all dbp: terms are defined in {VOCABULARY}.')
    return 0


if __name__ == '__main__':
    sys.exit(main())
