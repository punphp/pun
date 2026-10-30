<?php

namespace Probe;

class Upload
{
}

trait HasFiles
{
    /**
     * @return ($key is null ? array<string, Upload|Upload[]> : Upload|Upload[]|null)
     */
    public function file($key = null)
    {
        return null;
    }

    /**
     * @param  string  $key
     */
    public function hasFile($key)
    {
        if (! is_array($files = $this->file($key))) {
            $files = [$files];
        }

        return array_any($files, fn ($file) => $file instanceof Upload);
    }
}

class InClass
{
    /**
     * @return ($key is null ? array<string, Upload|Upload[]> : Upload|Upload[]|null)
     */
    public function file($key = null)
    {
        return null;
    }

    /**
     * @param  string  $key
     */
    public function hasFile($key)
    {
        if (! is_array($files = $this->file($key))) {
            $files = [$files];
        }

        return array_any($files, fn ($file) => $file instanceof Upload);
    }

    /**
     * @return Upload|Upload[]|null
     */
    public function plain()
    {
        return null;
    }

    public function hasPlain()
    {
        if (! is_array($files = $this->plain())) {
            $files = [$files];
        }

        return array_any($files, fn ($file) => $file instanceof Upload);
    }
}
-----
<?php

namespace Probe;

class Upload
{
}

trait HasFiles
{
    /**
     * @return ($key is null ? array<string, Upload|Upload[]> : Upload|Upload[]|null)
     */
    public function file($key = null)
    {
        return null;
    }

    /**
     * @param  string  $key
     */
    public function hasFile($key)
    {
        if (! is_array($files = $this->file($key))) {
            $files = [$files];
        }

        return array_any($files, fn ($file) => $file instanceof Upload);
    }
}

class InClass
{
    /**
     * @return ($key is null ? array<string, Upload|Upload[]> : Upload|Upload[]|null)
     */
    public function file($key = null)
    {
        return null;
    }

    /**
     * @param  string  $key
     */
    public function hasFile($key)
    {
        if (! is_array($files = $this->file($key))) {
            $files = [$files];
        }

        return array_any($files, fn (?\Probe\Upload $file) => $file instanceof Upload);
    }

    /**
     * @return Upload|Upload[]|null
     */
    public function plain()
    {
        return null;
    }

    public function hasPlain()
    {
        if (! is_array($files = $this->plain())) {
            $files = [$files];
        }

        return array_any($files, fn (?\Probe\Upload $file) => $file instanceof Upload);
    }
}
