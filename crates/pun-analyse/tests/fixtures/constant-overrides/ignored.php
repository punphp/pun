<?php
namespace ConstantOverrides\Ignored;

class ParentConstants
{
    public const int NATIVE = 1;
    public const int MISSING = 1;
    /** @var int */
    public const DOC = 1;
}

class ChildConstants extends ParentConstants
{
    public const string NATIVE = 's'; // @phpstan-ignore classConstant.nativeType
    public const MISSING = 1; // @phpstan-ignore classConstant.missingNativeType
    /** @var string */
    public const DOC = 's'; // @phpstan-ignore classConstant.type
}
