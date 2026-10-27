<?php

namespace Pun\Oracle\UnusedPrivate;

enum EnumParamCaller
{
    case One;
}

final class EnumParamDynamic
{
    public function run(EnumParamCaller $caller, $m)
    {
        return [[$caller, $m], [EnumParamCaller::One, $m]];
    }

    private function kept()
    {
    }
}

final class ConstantNames
{
    const NAME = 'byConstant';

    public function run()
    {
        return [[self::class, 'viaSelfClass'], [$this, self::NAME]];
    }

    private function viaSelfClass()
    {
    }

    private function byConstant()
    {
    }

    private function unused()
    {
    }
}

final class ExpressionClassName
{
    public function run($obj)
    {
        return [$this, $obj::class];
    }

    private function kept()
    {
    }
}
-----
<?php

namespace Pun\Oracle\UnusedPrivate;

enum EnumParamCaller
{
    case One;
}

final class EnumParamDynamic
{
    public function run(EnumParamCaller $caller, $m)
    {
        return [[$caller, $m], [EnumParamCaller::One, $m]];
    }

    private function kept()
    {
    }
}

final class ConstantNames
{
    const NAME = 'byConstant';

    public function run()
    {
        return [[self::class, 'viaSelfClass'], [$this, self::NAME]];
    }

    private function viaSelfClass()
    {
    }
}

final class ExpressionClassName
{
    public function run($obj)
    {
        return [$this, $obj::class];
    }

    private function kept()
    {
    }
}
