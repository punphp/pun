<?php

final class NativeArrayLocals
{
    public function filtered(array $credentials)
    {
        $credentials = array_filter($credentials, fn ($key) => $key !== 'password', ARRAY_FILTER_USE_KEY);

        return empty($credentials);
    }

    public function reassignedUntyped($jobs)
    {
        $jobs = array_values((array) $jobs);

        return ! empty($jobs);
    }

    public function conditionallyReassigned($relations)
    {
        if (is_string($relations)) {
            $relations = func_get_args();
        }

        return empty($relations);
    }

    public function fromMixedMerge($notification, ?array $channels)
    {
        $list = $channels ?: $notification->via();
        if (method_exists($notification, 'shouldSend')) {
            $list = array_filter($list);
        }

        return empty($list);
    }

    public function readInForeach(array $items)
    {
        foreach ($items as $item) {
        }

        return empty($items);
    }
}
-----
<?php

final class NativeArrayLocals
{
    public function filtered(array $credentials)
    {
        $credentials = array_filter($credentials, fn ($key) => $key !== 'password', ARRAY_FILTER_USE_KEY);

        return $credentials === [];
    }

    public function reassignedUntyped($jobs)
    {
        $jobs = array_values((array) $jobs);

        return $jobs !== [];
    }

    public function conditionallyReassigned($relations)
    {
        if (is_string($relations)) {
            $relations = func_get_args();
        }

        return empty($relations);
    }

    public function fromMixedMerge($notification, ?array $channels)
    {
        $list = $channels ?: $notification->via();
        if (method_exists($notification, 'shouldSend')) {
            $list = array_filter($list);
        }

        return empty($list);
    }

    public function readInForeach(array $items)
    {
        foreach ($items as $item) {
        }

        return $items === [];
    }
}
