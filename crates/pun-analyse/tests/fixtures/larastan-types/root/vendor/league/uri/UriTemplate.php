<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri;

use BackedEnum;
use Deprecated;
use League\Uri\Contracts\UriException;
use League\Uri\Contracts\UriInterface;
use League\Uri\Exceptions\MissingFeature;
use League\Uri\Exceptions\SyntaxError;
use League\Uri\UriTemplate\ExtractionResult;
use League\Uri\UriTemplate\Template;
use League\Uri\UriTemplate\TemplateCanNotBeExpanded;
use League\Uri\UriTemplate\VariableBag;
use League\Uri\UriTemplate\VariableCanNotBeExtracted;
use Psr\Http\Message\UriFactoryInterface;
use Psr\Http\Message\UriInterface as Psr7UriInterface;
use Stringable;
use Uri\InvalidUriException;
use Uri\Rfc3986\Uri as Rfc3986Uri;
use Uri\WhatWg\InvalidUrlException;
use Uri\WhatWg\Url as WhatWgUrl;
use Uri\WhatWg\UrlValidationError;

use function array_fill_keys;
use function array_key_exists;
use function class_exists;

/**
 * Defines the URI Template syntax and the process for expanding a URI Template into a URI reference.
 *
 * @link    https://tools.ietf.org/html/rfc6570
 * @package League\Uri
 * @author  Ignace Nyamagana Butera <nyamsprod@gmail.com>
 * @since   6.1.0
 *
 * @phpstan-import-type InputValue from VariableBag
 */
final class UriTemplate implements Stringable
{
    private readonly Template $template;
    private readonly VariableBag $defaultVariables;

    /**
     * @throws SyntaxError if the template syntax is invalid
     * @throws TemplateCanNotBeExpanded if the template or the variables are invalid
     */
    public function __construct(BackedEnum|Stringable|string $template, iterable|ExtractionResult $defaultVariables = [])
    {}

    private function filterVariables(iterable|ExtractionResult $variables): VariableBag
    {}

    /**
     * Returns the string representation of the UriTemplate.
     */
    public function __toString(): string
    {}

    /**
     * Returns the distinct variables placeholders used in the template.
     *
     * @return array<string>
     */
    public function getVariableNames(): array
    {}

    /**
     * @return array<string, InputValue>
     */
    public function getDefaultVariables(): array
    {}

    /**
     * Tells whether the UriTemplate contains variables.
     */
    public function hasVariable(): bool
    {}

    /**
     * Returns a new instance with the updated default variables.
     *
     * This method MUST retain the state of the current instance, and return
     * an instance that contains the modified default variables.
     *
     * If present, variables whose name is not part of the current template
     * possible variable names are removed.
     *
     * @throws TemplateCanNotBeExpanded if the variables are invalid
     */
    public function withDefaultVariables(iterable|ExtractionResult $defaultVariables): self
    {}

    private function templateExpanded(iterable|ExtractionResult $variables = []): string
    {}

    private function templateExpandedOrFail(iterable|ExtractionResult $variables = []): string
    {}

    /**
     * @throws TemplateCanNotBeExpanded if the variables are invalid
     * @throws UriException if the resulting expansion cannot be converted to a UriInterface instance
     */
    public function expand(iterable|ExtractionResult $variables = [], Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUri = null): UriInterface
    {}

    /**
     * @throws MissingFeature if no Uri\Rfc3986\Uri class is found
     * @throws TemplateCanNotBeExpanded if the variables are invalid
     * @throws InvalidUriException if the base URI cannot be converted to a Uri\Rfc3986\Uri instance
     * @throws InvalidUriException if the resulting expansion cannot be converted to a Uri\Rfc3986\Uri instance
     */
    public function expandToUri(iterable|ExtractionResult $variables = [], Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUri = null): Rfc3986Uri
    {}

    /**
     * @param list<UrlValidationError>|null $errors
     *
     * @throws TemplateCanNotBeExpanded if the variables are invalid
     * @throws InvalidUrlException if the base URI cannot be converted to a Uri\Whatwg\Url instance
     * @throws InvalidUrlException if the resulting expansion cannot be converted to a Uri\Whatwg\Url instance
     * @throws MissingFeature if no Uri\Whatwg\Url class is found
     */
    public function expandToUrl(iterable|ExtractionResult $variables = [], Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUrl = null, array|null &$errors = []): WhatWgUrl
    {}

    /**
     * @throws TemplateCanNotBeExpanded if the variables are invalid
     * @throws UriException if the resulting expansion cannot be converted to a UriInterface instance
     */
    public function expandToPsr7Uri(
        iterable|ExtractionResult $variables = [],
        Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUrl = null,
        UriFactoryInterface $uriFactory = new HttpFactory()
    ): Psr7UriInterface {}

    /**
     * @throws TemplateCanNotBeExpanded if the variables are invalid or missing
     * @throws UriException if the resulting expansion cannot be converted to a UriInterface instance
     */
    public function expandOrFail(iterable|ExtractionResult $variables = [], Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUri = null): UriInterface
    {}

    /**
     * @throws MissingFeature if no Uri\Rfc3986\Uri class is found
     * @throws TemplateCanNotBeExpanded if the variables are invalid
     * @throws InvalidUriException if the base URI cannot be converted to a Uri\Rfc3986\Uri instance
     * @throws InvalidUriException if the resulting expansion cannot be converted to a Uri\Rfc3986\Uri instance
     */
    public function expandToUriOrFail(iterable|ExtractionResult $variables = [], Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUri = null): Rfc3986Uri
    {}

    /**
     * @param list<UrlValidationError>|null $errors
     *
     * @throws MissingFeature if no Uri\Whatwg\Url class is found
     * @throws TemplateCanNotBeExpanded if the variables are invalid
     * @throws InvalidUrlException if the base URI cannot be converted to a Uri\Whatwg\Url instance
     * @throws InvalidUrlException if the resulting expansion cannot be converted to a Uri\Whatwg\Url instance
     */
    public function expandToUrlOrFail(iterable|ExtractionResult $variables = [], Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUrl = null, array|null &$errors = []): WhatWgUrl
    {}

    /**
     * @throws TemplateCanNotBeExpanded if the variables are invalid
     * @throws UriException if the resulting expansion cannot be converted to a UriInterface instance
     */
    public function expandToPsr7UriOrFail(
        iterable|ExtractionResult $variables = [],
        Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $baseUrl = null,
        UriFactoryInterface $uriFactory = new HttpFactory()
    ): Psr7UriInterface {}

    /**
     * @throws InvalidUrlException
     */
    private function newWhatWgUrl(Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $url = null): ?WhatWgUrl
    {}

    /**
     * @throws InvalidUriException
     */
    private function newRfc3986Uri(Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string|null $uri = null): ?Rfc3986Uri
    {}

    /**
     * Extracts the variables from a URI using the template.
     */
    public function extract(Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string $uri): ExtractionResult
    {}

    /**
     * @throws VariableCanNotBeExtracted if the URI cannot be extracted using the template
     */
    public function extractOrFail(Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string $uri): ExtractionResult
    {}

    /**
     * Returns whether the URI matches the template.
     */
    public function match(Rfc3986Uri|WhatWgUrl|BackedEnum|Stringable|string $uri): bool
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @deprecated Since version 7.6.0
     * @codeCoverageIgnore
     * @see UriTemplate::toString()
     *
     * Create a new instance from the environment.
     */
    #[Deprecated(message:'use League\Uri\UriTemplate::__toString() instead', since:'league/uri:7.6.0')]
    public function getTemplate(): string
    {}
}
