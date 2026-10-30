<?php
namespace TraitMagicSource;
trait MagicInputs {
    public const FILE = __FILE__;
    public const DIR = __DIR__;
    public const NS = __NAMESPACE__;
    public const CLASS_NAME = __CLASS__;
    public const TRAIT_NAME = __TRAIT__;
    public const LABEL = self::class;
}
