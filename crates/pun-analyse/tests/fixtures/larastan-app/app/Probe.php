<?php

namespace App\Probe;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Support\Collection;
use Illuminate\Support\Enumerable;
use Illuminate\Support\Facades\Auth;
use Illuminate\Support\Facades\Storage;

use function PHPStan\dumpType;

final class Probe
{
    /**
     * @param  Collection<string, string|null>  $names
     * @param  Collection<int, int|string>  $values
     * @param  Collection<int, mixed>  $mixed
     * @param  Enumerable<int, string>  $enumerable
     * @param  Collection<int, int|float|bool|string|object|null>  $scalars
     * @param  \ArrayIterator<int, string>  $iterator
     */
    public function types(Application $app, Collection $names, Collection $values, Collection $mixed, Enumerable $enumerable, Collection $scalars, \ArrayIterator $iterator): void
    {
        dumpType(auth());
        dumpType(auth()->user());
        dumpType(auth()->guard('web')->user());
        dumpType(auth('web'));
        dumpType(Auth::user());
        dumpType(now());
        dumpType(today());
        dumpType(Storage::disk('local'));
        dumpType($app->isProduction());
        dumpType($app->environment());
        dumpType(collect());
        dumpType(collect([]));
        dumpType($names->filter());
        dumpType($names->filter(fn (?string $name): bool => $name !== null));
        dumpType($names->reject(fn (?string $name): bool => $name === null || $name === ''));
        dumpType($values->filter(function (int|string $value): bool {
            return ! is_int($value);
        }));
        dumpType($mixed->filter(fn (mixed $value): bool => is_string($value) && $value !== ''));
        dumpType($values->where('a', 1));
        dumpType($enumerable->pluck(fn (string $value): int => strlen($value)));
        dumpType($scalars->filter(fn ($value): bool => is_float($value) || is_bool($value)));
        dumpType($scalars->reject(fn ($value): bool => is_null($value) || is_object($value)));
        dumpType($scalars->filter(fn ($value): bool => $value === true));
        dumpType($scalars->filter(fn ($value) => $value));
        dumpType($scalars->filter(fn ($value): bool => \is_int($value) and $value !== 0));
        dumpType($scalars->filter(fn ($value, $key): bool => $key > 1));
        dumpType($scalars->filter('is_string'));
        dumpType($scalars->filter(function ($value): bool {
            $copy = $value;

            return is_string($copy);
        }));
        dumpType($scalars->reject());
        dumpType($scalars->where(fn ($value): bool => is_string($value)));
        dumpType(collect($iterator));
        dumpType(__());
        dumpType(__('messages.welcome'));
    }
    public function helpers(\Illuminate\Http\Request $request, \Illuminate\Container\Container $container): void
    {
        dumpType(app());
        dumpType(app('auth'));
        dumpType(resolve('auth'));
        dumpType($container->make('auth'));
        dumpType(value(5));
        dumpType(value(fn (): int => 1));
        dumpType(validator());
        dumpType(validator([], []));
        dumpType(str('a'));
        dumpType(str());
        dumpType(tap(new \Exception()));
        dumpType(tap(new \Exception())->getCode());
        dumpType($request->file());
        dumpType($request->file('a'));
        dumpType($request->route());
        dumpType($request->route('a', 'b'));
        dumpType($request->query('a'));
    }
}
