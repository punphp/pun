<?php
namespace AliasEdge;
#[\Attribute]
class Real {}
class Plain {}

function dormant(): void {
    \class_alias(Real::class, Dormant::class);
}
