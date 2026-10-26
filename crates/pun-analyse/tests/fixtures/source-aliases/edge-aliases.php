<?php
namespace AliasEdge {
    if (false) {
        \class_alias(Real::class, Dead::class);
    }
    if (!class_exists(Guarded::class, false)) {
        \class_alias(Real::class, Guarded::class);
    }
    $GLOBALS['aliasResults']['twin-real'] = @\class_alias(Real::class, 'AliasEdge\Twin');
    $GLOBALS['aliasResults']['twin-plain'] = @\class_alias(Plain::class, 'AliasEdge\tWIN');
    $GLOBALS['aliasResults']['same-name'] = @\class_alias(Real::class, 'AliasEdge\rEAL');
    $GLOBALS['aliasResults']['missing'] = @\class_alias('AliasEdge\NoSuch', Missing::class);
}
namespace AliasEdge\Shadow {
    function class_alias(string $class, string $alias): bool { return false; }
    class_alias(\AliasEdge\Real::class, ShadowAlias::class);
}
