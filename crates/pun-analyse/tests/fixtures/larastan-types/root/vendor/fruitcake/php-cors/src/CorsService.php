<?php

/*
 * This file is part of fruitcake/php-cors and was originally part of asm89/stack-cors
 *
 * (c) Alexander <iam.asm89@gmail.com>
 * (c) Barryvdh <barryvdh@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Fruitcake\Cors;

use Fruitcake\Cors\Exceptions\InvalidOptionException;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;

/**
 * @phpstan-type CorsInputOptions array{
 *  'allowedOrigins'?: string[],
 *  'allowedOriginsPatterns'?: string[],
 *  'supportsCredentials'?: bool,
 *  'allowedHeaders'?: string[],
 *  'allowedMethods'?: string[],
 *  'exposedHeaders'?: string[]|false,
 *  'maxAge'?: int|bool|null,
 *  'allowed_origins'?: string[],
 *  'allowed_origins_patterns'?: string[],
 *  'supports_credentials'?: bool,
 *  'allowed_headers'?: string[],
 *  'allowed_methods'?: string[],
 *  'exposed_headers'?: string[]|false,
 *  'max_age'?: int|bool|null
 * }
 *
 */
class CorsService
{
    /** @var string[]  */
    private array $allowedOrigins = [];
    /** @var string[] */
    private array $allowedOriginsPatterns = [];
    /** @var string[] */
    private array $allowedMethods = [];
    /** @var string[] */
    private array $allowedHeaders = [];
    /** @var string[] */
    private array $exposedHeaders = [];
    private bool $supportsCredentials = false;
    private ?int $maxAge = 0;

    private bool $allowAllOrigins = false;
    private bool $allowAllMethods = false;
    private bool $allowAllHeaders = false;

    /**
     * @param CorsInputOptions $options
     */
    public function __construct(array $options = [])
    {}

    /**
     * @param CorsInputOptions $options
     */
    public function setOptions(array $options): void
    {}

    private function normalizeOptions(): void
    {}

    /**
     * Create a pattern for a wildcard, based on Str::is() from Laravel
     *
     * @see https://github.com/laravel/framework/blob/5.5/src/Illuminate/Support/Str.php
     * @param string $pattern
     * @return string
     */
    private function convertWildcardToPattern($pattern)
    {}

    public function isCorsRequest(Request $request): bool
    {}

    public function isPreflightRequest(Request $request): bool
    {}

    public function handlePreflightRequest(Request $request): Response
    {}

    public function addPreflightRequestHeaders(Response $response, Request $request): Response
    {}

    public function isOriginAllowed(Request $request): bool
    {}

    public function addActualRequestHeaders(Response $response, Request $request): Response
    {}

    private function configureAllowedOrigin(Response $response, Request $request): void
    {}

    private function isSingleOriginAllowed(): bool
    {}

    private function configureAllowedMethods(Response $response, Request $request): void
    {}

    private function configureAllowedHeaders(Response $response, Request $request): void
    {}

    private function configureAllowCredentials(Response $response, Request $request): void
    {}

    private function configureExposedHeaders(Response $response, Request $request): void
    {}

    private function configureMaxAge(Response $response, Request $request): void
    {}

    public function varyHeader(Response $response, string $header): Response
    {}
}
