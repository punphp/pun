<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\ErrorHandler\ErrorRenderer;

use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\RequestStack;
use Symfony\Component\Routing\Generator\UrlGeneratorInterface;

/**
 * Formats debug file links.
 *
 * @author Jérémy Romey <jeremy@free-agent.fr>
 *
 * @final
 */
class FileLinkFormatter
{
    private array|false $fileLinkFormat;

    /**
     * @param string|\Closure $urlFormat The URL format, or a closure that returns it on-demand
     */
    public function __construct(
        string|array|null $fileLinkFormat = null,
        private ?RequestStack $requestStack = null,
        private ?string $baseDir = null,
        private string|\Closure|null $urlFormat = null,
    ) {}

    public function format(string $file, int $line): string|false
    {}

    public function __serialize(): array
    {}

    /**
     * @internal
     */
    public static function generateUrlFormat(UrlGeneratorInterface $router, string $routeName, string $queryString): ?string
    {}

    private function getFileLinkFormat(): array|false
    {}
}
