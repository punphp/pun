<?php
namespace SelectedConstantOverrides;

class PublicParent { public const VALUE = 1; }
class ProtectedParent { protected const VALUE = 1; }

class LastPublic extends PublicParent {
    private const VALUE = 2;
    public const VALUE = 3;
}
class LastPrivate extends PublicParent {
    public const VALUE = 4;
    private const VALUE = 5;
}
class LastProtected extends PublicParent {
    public const VALUE = 6;
    protected const VALUE = 7;
}
class ProtectedToPublic extends ProtectedParent {
    private const VALUE = 8;
    public const VALUE = 9;
}
class ProtectedToPrivate extends ProtectedParent {
    public const VALUE = 10;
    private const VALUE = 11;
}
class ProtectedToProtected extends ProtectedParent {
    private const VALUE = 12;
    protected const VALUE = 13;
}

trait PublicLast {
    private const VALUE = 17;
    public const VALUE = 19;
}
class PublicTraitBase { use PublicLast; }
class PublicTraitReuse extends PublicTraitBase { use PublicLast; }

trait PrivateLast {
    public const VALUE = 23;
    private const VALUE = 29;
}
class PrivateTraitBase { use PrivateLast; }
class PrivateTraitReuse extends PrivateTraitBase { use PrivateLast; }

trait ProtectedLast {
    private const VALUE = 31;
    protected const VALUE = 37;
}
class ProtectedTraitBase { use ProtectedLast; }
class ProtectedTraitReuse extends ProtectedTraitBase { use ProtectedLast; }
