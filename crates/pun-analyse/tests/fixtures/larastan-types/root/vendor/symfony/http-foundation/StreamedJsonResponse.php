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
 * StreamedJsonResponse represents a streamed HTTP response for JSON.
 *
 * A StreamedJsonResponse uses a structure and generics to create an
 * efficient resource-saving JSON response.
 *
 * It is recommended to use flush() function after a specific number of items to directly stream the data.
 *
 * @see flush()
 *
 * @author Alexander Schranz <alexander@sulu.io>
 *
 * Example usage:
 *
 *     function loadArticles(): \Generator
 *         // some streamed loading
 *         yield ['title' => 'Article 1'];
 *         yield ['title' => 'Article 2'];
 *         yield ['title' => 'Article 3'];
 *         // recommended to use flush() after every specific number of items
 *     }),
 *
 *     $response = new StreamedJsonResponse(
 *         // json structure with generators in which will be streamed
 *         [
 *             '_embedded' => [
 *                 'articles' => loadArticles(), // any generator which you want to stream as list of data
 *             ],
 *         ],
 *     );
 */
class StreamedJsonResponse extends StreamedResponse
{
    private const PLACEHOLDER = '__symfony_json__';

    /**
     * @param mixed[]                        $data            JSON Data containing PHP generators which will be streamed as list of data or a Generator
     * @param int                            $status          The HTTP status code (200 "OK" by default)
     * @param array<string, string|string[]> $headers         An array of HTTP headers
     * @param int                            $encodingOptions Flags for the json_encode() function
     */
    public function __construct(
        private readonly iterable $data,
        int $status = 200,
        array $headers = [],
        private int $encodingOptions = JsonResponse::DEFAULT_ENCODING_OPTIONS,
    ) {}

    private function stream(): void
    {}

    private function streamData(mixed $data, int $jsonEncodingOptions, int $keyEncodingOptions): void
    {}

    private function streamArray(array $data, int $jsonEncodingOptions, int $keyEncodingOptions): void
    {}

    private function streamIterable(iterable $iterable, int $jsonEncodingOptions, int $keyEncodingOptions): void
    {}
}
