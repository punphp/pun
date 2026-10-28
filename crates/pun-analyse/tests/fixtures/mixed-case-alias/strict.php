<?php
namespace MixedAliasStrict;

trait Body { public function OriginalMethod(string $old): void {} }
class Base { use Body { OriginalMethod as MixedAlias; } }
class Middle extends Base {}
class Matching extends Middle { public function MixedAlias(string $old): void {} }
class WrongCase extends Middle { public function mixedalias(string $old): void {} }
class Original extends Middle { public function OriginalMethod(string $old): void {} }
