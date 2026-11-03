<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Routing\Generator;

use Psr\Log\LoggerInterface;
use Symfony\Component\Routing\Exception\InvalidParameterException;
use Symfony\Component\Routing\Exception\MissingMandatoryParametersException;
use Symfony\Component\Routing\Exception\RouteNotFoundException;
use Symfony\Component\Routing\RequestContext;
use Symfony\Component\Routing\RouteCollection;

/**
 * UrlGenerator can generate a URL or a path for any route in the RouteCollection
 * based on the passed parameters.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Tobias Schultze <http://tobion.de>
 */
class UrlGenerator implements UrlGeneratorInterface, ConfigurableRequirementsInterface
{
    private const QUERY_FRAGMENT_DECODED = [
        // RFC 3986 explicitly allows those in the query/fragment to reference other URIs unencoded
        '%2F' => '/',
        '%252F' => '%2F',
        '%3F' => '?',
        // reserved chars that have no special meaning for HTTP URIs in a query or fragment
        // this excludes esp. "&", "=" and also "+" because PHP would treat it as a space (form-encoded)
        '%40' => '@',
        '%3A' => ':',
        '%21' => '!',
        '%3B' => ';',
        '%2C' => ',',
        '%2A' => '*',
    ];

    protected ?bool $strictRequirements = true;

    /**
     * This array defines the characters (besides alphanumeric ones) that will not be percent-encoded in the path segment of the generated URL.
     *
     * PHP's rawurlencode() encodes all chars except "a-zA-Z0-9-._~" according to RFC 3986. But we want to allow some chars
     * to be used in their literal form (reasons below). Other chars inside the path must of course be encoded, e.g.
     * "?" and "#" (would be interpreted wrongly as query and fragment identifier),
     * "'" and """ (are used as delimiters in HTML).
     */
    protected array $decodedChars = [
        // the slash can be used to designate a hierarchical structure and we want allow using it with this meaning
        // some webservers don't allow the slash in encoded form in the path for security reasons anyway
        // see http://stackoverflow.com/questions/4069002/http-400-if-2f-part-of-get-url-in-jboss
        '%2F' => '/',
        '%252F' => '%2F',
        // the following chars are general delimiters in the URI specification but have only special meaning in the authority component
        // so they can safely be used in the path in unencoded form
        '%40' => '@',
        '%3A' => ':',
        // these chars are only sub-delimiters that have no predefined meaning and can therefore be used literally
        // so URI producing applications can use these chars to delimit subcomponents in a path segment without being encoded for better readability
        '%3B' => ';',
        '%2C' => ',',
        '%3D' => '=',
        '%2B' => '+',
        '%21' => '!',
        '%2A' => '*',
        '%7C' => '|',
    ];

    public function __construct(
        protected RouteCollection $routes,
        protected RequestContext $context,
        protected ?LoggerInterface $logger = null,
        private ?string $defaultLocale = null,
    ) {}

    public function setContext(RequestContext $context): void
    {}

    public function getContext(): RequestContext
    {}

    public function setStrictRequirements(?bool $enabled): void
    {}

    public function isStrictRequirements(): ?bool
    {}

    public function generate(string $name, array $parameters = [], int $referenceType = self::ABSOLUTE_PATH): string
    {}

    /**
     * @throws MissingMandatoryParametersException When some parameters are missing that are mandatory for the route
     * @throws InvalidParameterException           When a parameter value for a placeholder is not correct because
     *                                             it does not match the requirement
     */
    protected function doGenerate(array $variables, array $defaults, array $requirements, array $tokens, array $parameters, string $name, int $referenceType, array $hostTokens, array $requiredSchemes = []): string
    {}

    /**
     * Returns the target path as relative reference from the base path.
     *
     * Only the URIs path component (no schema, host etc.) is relevant and must be given, starting with a slash.
     * Both paths must be absolute and not contain relative parts.
     * Relative URLs from one resource to another are useful when generating self-contained downloadable document archives.
     * Furthermore, they can be used to reduce the link size in documents.
     *
     * Example target paths, given a base path of "/a/b/c/d":
     * - "/a/b/c/d"     -> ""
     * - "/a/b/c/"      -> "./"
     * - "/a/b/"        -> "../"
     * - "/a/b/c/other" -> "other"
     * - "/a/x/y"       -> "../../x/y"
     *
     * @param string $basePath   The base path
     * @param string $targetPath The target path
     */
    public static function getRelativePath(string $basePath, string $targetPath): string
    {}
}
