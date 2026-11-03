<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation;

/**
 * Response represents an HTTP response in JSON format.
 *
 * Note that this class does not force the returned JSON content to be an
 * object. It is however recommended that you do return an object as it
 * protects yourself against XSSI and JSON-JavaScript Hijacking.
 *
 * @see https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/AJAX_Security_Cheat_Sheet.md#always-return-json-with-an-object-on-the-outside
 *
 * @author Igor Wiedler <igor@wiedler.ch>
 */
class JsonResponse extends Response
{
    protected mixed $data;
    protected ?string $callback = null;

    // Encode <, >, ', &, and " characters in the JSON, making it also safe to be embedded into HTML.
    // 15 === JSON_HEX_TAG | JSON_HEX_APOS | JSON_HEX_AMP | JSON_HEX_QUOT
    public const DEFAULT_ENCODING_OPTIONS = 15;

    protected int $encodingOptions = self::DEFAULT_ENCODING_OPTIONS;

    /**
     * @param bool $json If the data is already a JSON string
     */
    public function __construct(mixed $data = null, int $status = 200, array $headers = [], bool $json = false)
    {}

    /**
     * Factory method for chainability.
     *
     * Example:
     *
     *     return JsonResponse::fromJsonString('{"key": "value"}')
     *         ->setSharedMaxAge(300);
     *
     * @param string $data    The JSON response string
     * @param int    $status  The response status code (200 "OK" by default)
     * @param array  $headers An array of response headers
     */
    public static function fromJsonString(string $data, int $status = 200, array $headers = []): static
    {}

    /**
     * Sets the JSONP callback.
     *
     * @param string|null $callback The JSONP callback or null to use none
     *
     * @return $this
     *
     * @throws \InvalidArgumentException When the callback name is not valid
     */
    public function setCallback(?string $callback): static
    {}

    /**
     * Sets a raw string containing a JSON document to be sent.
     *
     * @return $this
     */
    public function setJson(string $json): static
    {}

    /**
     * Sets the data to be sent as JSON.
     *
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function setData(mixed $data = []): static
    {}

    /**
     * Returns options used while encoding data to JSON.
     */
    public function getEncodingOptions(): int
    {}

    /**
     * Sets options used while encoding data to JSON.
     *
     * @return $this
     */
    public function setEncodingOptions(int $encodingOptions): static
    {}

    /**
     * Updates the content and headers according to the JSON data and callback.
     *
     * @return $this
     */
    protected function update(): static
    {}
}
