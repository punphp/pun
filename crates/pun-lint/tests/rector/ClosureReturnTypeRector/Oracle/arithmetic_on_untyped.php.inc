<?php

final class Timing
{
    public function run(callable $callback)
    {
        $measure = function () use ($callback) {
            $start = hrtime(true);

            $callback();

            return (hrtime(true) - $start) / 1_000_000;
        };

        $scaled = fn () => microtime(true) * 1000;

        $wrapped = fn () => (microtime(true) * 1000);

        $wrappedMeasure = function () {
            $start = hrtime(true);

            return ((hrtime(true) - $start) / 1_000_000);
        };

        return [$measure, $scaled, $wrapped, $wrappedMeasure];
    }
}
-----
<?php

final class Timing
{
    public function run(callable $callback)
    {
        $measure = function () use ($callback): int|float {
            $start = hrtime(true);

            $callback();

            return (hrtime(true) - $start) / 1_000_000;
        };

        $scaled = fn () => microtime(true) * 1000;

        $wrapped = fn () => (microtime(true) * 1000);

        $wrappedMeasure = function (): int|float {
            $start = hrtime(true);

            return ((hrtime(true) - $start) / 1_000_000);
        };

        return [$measure, $scaled, $wrapped, $wrappedMeasure];
    }
}
