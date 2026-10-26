<?php
// Mixed-case directory names make path generalization independent of the checkout's casing.
namespace TraitMagicConsumer;
class First { use \TraitMagicSource\MagicInputs; }
class Second { use \TraitMagicSource\MagicInputs; }
class Child extends First { use \TraitMagicSource\MagicInputs; }
\PHPStan\dumpType(First::FILE);
\PHPStan\dumpType(First::DIR);
\PHPStan\dumpType(First::NS);
\PHPStan\dumpType(First::CLASS_NAME);
\PHPStan\dumpType(First::TRAIT_NAME);
\PHPStan\dumpType(First::LABEL);
\PHPStan\dumpType(Second::CLASS_NAME);
\PHPStan\dumpType(Child::CLASS_NAME);
\PHPStan\dumpType(\TraitMagicScanned\Owner::FILE);
\PHPStan\dumpType(\TraitMagicScanned\Child::FILE);
\PHPStan\dumpType(\TraitMagicScanned\Child::NS);
\PHPStan\dumpType(\TraitMagicScanned\Child::CLASS_NAME);
