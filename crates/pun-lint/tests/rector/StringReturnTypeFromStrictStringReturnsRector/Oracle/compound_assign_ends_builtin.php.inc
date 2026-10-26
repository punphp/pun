<?php

final class QueryStrings
{
    /**
     * @param  string  $uri
     */
    protected function addQueryString($uri, array $parameters)
    {
        if (! is_null($fragment = parse_url($uri, PHP_URL_FRAGMENT))) {
            $uri = preg_replace('/#.*/', '', $uri);
        }

        $uri .= '?'.http_build_query($parameters);

        return is_null($fragment) ? $uri : $uri."#{$fragment}";
    }

    protected function replaced($uri)
    {
        $uri = preg_replace('/#.*/', '', $uri);

        return $uri.'#';
    }
}
-----
<?php

final class QueryStrings
{
    /**
     * @param  string  $uri
     */
    protected function addQueryString($uri, array $parameters): string
    {
        if (! is_null($fragment = parse_url($uri, PHP_URL_FRAGMENT))) {
            $uri = preg_replace('/#.*/', '', $uri);
        }

        $uri .= '?'.http_build_query($parameters);

        return is_null($fragment) ? $uri : $uri."#{$fragment}";
    }

    protected function replaced($uri)
    {
        $uri = preg_replace('/#.*/', '', $uri);

        return $uri.'#';
    }
}
