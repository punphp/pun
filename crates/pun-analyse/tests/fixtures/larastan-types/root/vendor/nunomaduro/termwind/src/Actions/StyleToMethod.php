<?php

declare(strict_types=1);

namespace Termwind\Actions;

use Termwind\Exceptions\StyleNotFound;
use Termwind\Repositories\Styles as StyleRepository;
use Termwind\Terminal;
use Termwind\ValueObjects\Styles;

/**
 * @internal
 */
final class StyleToMethod
{
    /**
     * Finds if there is any media query on the style class.
     */
    private const MEDIA_QUERIES_REGEX = "/^(sm|md|lg|xl|2xl)\:(.*)/";

    /**
     * Defines the Media Query Breakpoints.
     */
    public const MEDIA_QUERY_BREAKPOINTS = [
        'sm' => 64,
        'md' => 76,
        'lg' => 102,
        'xl' => 128,
        '2xl' => 153,
    ];

    /**
     * Creates a new action instance.
     */
    public function __construct(
        private Styles $styles,
        private string $style,
    ) {}

    /**
     * Applies multiple styles to the given styles.
     */
    public static function multiple(Styles $styles, string $stylesString): Styles
    {}

    /**
     * Converts the given style to a method name.
     */
    public function __invoke(string|int ...$arguments): Styles
    {}

    /**
     * Sorts all the styles based on the correct render order.
     *
     * @param  string[]  $styles
     * @return string[]
     */
    private static function sortStyles(array $styles): array
    {}

    /**
     * Applies the media query if exists.
     */
    private function applyMediaQuery(string $method): string
    {}
}
