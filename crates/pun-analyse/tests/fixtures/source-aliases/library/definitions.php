<?php
namespace AliasProbe;

#[\Attribute(\Attribute::TARGET_FUNCTION)]
class Marker {
    public const VALUE = 1;
    public static function ping(): void {}
}

class Plain {}

use Attribute as ImportedAttribute;
#[ImportedAttribute]
class ViaImport {}

#[\attribute]
class ViaLowerNative {}

#[Meta]
class ViaMeta {}
