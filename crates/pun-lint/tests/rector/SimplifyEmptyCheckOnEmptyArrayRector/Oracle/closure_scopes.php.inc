<?php

final class ClosureScopes
{
    public function run()
    {
        $typed = function (array $items) {
            return empty($items);
        };
        $reassigned = function ($items) {
            $items = array_values((array) $items);

            return empty($items);
        };
        $arrow = fn (array $items) => ! empty($items);
        $untypedArrow = fn ($items) => is_array($items) && empty($items);

        return [$typed, $reassigned, $arrow, $untypedArrow];
    }
}
-----
<?php

final class ClosureScopes
{
    public function run()
    {
        $typed = function (array $items) {
            return $items === [];
        };
        $reassigned = function ($items) {
            $items = array_values((array) $items);

            return $items === [];
        };
        $arrow = fn (array $items) => $items !== [];
        $untypedArrow = fn ($items) => is_array($items) && $items === [];

        return [$typed, $reassigned, $arrow, $untypedArrow];
    }
}
