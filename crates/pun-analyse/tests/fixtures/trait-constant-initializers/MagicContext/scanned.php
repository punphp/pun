<?php
// Retaining a scanned consumer for reflection must not admit its bodies to ordinary analysis.
namespace TraitMagicScanned;
class Owner { use \TraitMagicSource\MagicInputs; }
class Child extends Owner { use \TraitMagicSource\MagicInputs; }
