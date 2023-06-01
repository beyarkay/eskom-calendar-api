<div align="center">
  <h1 alt="Eskom-calendar-api: loadshedding in your shell">
    <img src="https://github.com/beyarkay/eskom-calendar/blob/main/imgs/header.png" alt="eskom-calendar-api: loadshedding in your shell">
  </h1>

  <a href="https://github.com/beyarkay/eskom-calendar-api">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/beyarkay/eskom-calendar-api?style=social">
  </a>

  <a href="https://twitter.com/intent/tweet?text=Hey%20@beyarkay%20I%20love%20eskom-calendar!">
    <img alt="Twitter share" src="https://img.shields.io/twitter/follow/beyarkay?label=Say%20hi%20on%20Twitter%21&style=social">
  </a>
  <br>
  <a href="https://ko-fi.com/beyarkay">
    <img alt="Support on Ko-Fi" src="https://img.shields.io/badge/Ko--Fi-Once%20off%20donation-ff5f5f">
  </a>

  <a href="https://patreon.com/user?u=91320409&utm_medium=clipboard_copy&utm_source=copyLink&utm_campaign=creatorshare_creator&utm_content=join_link">
    <img alt="Support on Patreon" src="https://img.shields.io/badge/Patreon-Recurring%20donation%20%E2%9D%A4%EF%B8%8F-red">
  </a>

  <p>
    This is the backend for <a href="https://github.com/beyarkay/eskom-calendar">eskom-calendar</a>'s API, hosted by <a href="https://www.shuttle.rs/">shuttle.rs🚀</a>.
  </p>
</div>

(This is still a WIP, but should be usable for non-critical purposes)

## How to use it

Proper docs will be coming in the near future, as the API is still on version `v0.0.1`, but 
the endpoint is `https://eskom-calendar-api.shuttleapp.rs/v0.0.1/`. Note that you can omit the
version to opt-in to the latest version of the endpoint. So `https://eskom-calendar-api.shuttleapp.rs/list_areas`
will list all the areas according to the latest version of the API. All versions are maintained, so if
your app is working fine on `v0.0.1`, you don't have to worry about it being deprecated/removed.

The basic layout is as follows (openAPI spec is being worked on):

### List all areas
```
curl https://eskom-calendar-api.shuttleapp.rs/v0.0.1/list_areas
```

### List areas matching regex
```
curl "https://eskom-calendar-api.shuttleapp.rs/v0.0.1/list_areas/(western|eastern)-cape"
```

### Get times when the power will be out for an area
```
curl https://eskom-calendar-api.shuttleapp.rs/v0.0.1/outages/western-cape-worscester
```

### Get the full schedule for an area

(This one's got all the correct data, but some of the niceties are still just placeholders)

```
curl https://eskom-calendar-api.shuttleapp.rs/v0.0.1/schedules/western-cape-worscester
```

## Attribution and using the data (commercially or otherwise)

The data (CSV and ICS files) are available under the
[CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) license. This means
that

> You are free to:
>
> - Share: copy and redistribute the material in any medium or format
>
> - Adapt: remix, transform, and build upon the material

But only if you follow the following terms:

> - Attribution: You must give appropriate credit, provide a link to the license,
>   and indicate if changes were made. You may do so in any reasonable manner, but
>   not in any way that suggests the licensor endorses you or your use.
>
> - NonCommercial: You may not use the material for commercial purposes.
>
> - ShareAlike: If you remix, transform, or build upon the material, you must
>   distribute your contributions under the same license as the original.
>
> - No additional restrictions: You may not apply legal terms or technological
>   measures that legally restrict others from doing anything the license permits.

Attribution should be done in a form similar to how
[StackOverflow](https://stackoverflow.blog/2009/06/25/attribution-required/) requires it.
Specifically:

- Visually indicate that the content is from EskomCalendar in some way.
  It doesn’t have to be obnoxious; a discreet text blurb is fine.

- When a specific calendar is being used, you must hyperlink directly to the website's
  version of that calendar (for example, https://eskomcalendar.co.za/ec?calendar=city-of-cape-town.ics).
  You do not have to present the full URL to the user, so long as it is clear that it is an EskomCalendar
  URL.

- If no specific calendar is being used, or the `machine_friendly.csv` file is being used, you must either
  hyperlink *directly* to the project website (https://eskomcalendar.co.za) or *directly* to the
  project github page (https://github.com/beyarkay/eskom-calendar)

By *directly*, we mean each hyperlink must point directly to the specified URL in standard HTML
visible even with JavaScript disabled, and not use a tinyurl or any other form of obfuscation
or redirection. Furthermore, the links must not be
[nofollowed](https://googleblog.blogspot.com/2005/01/preventing-comment-spam.html).


## Contributors

[Boyd Kane](https://github.com/beyarkay) is the creator and maintainer. Reach out
on [twitter](https://twitter.com/beyarkay) if you want to chat in private, otherwise
[open an issue](https://github.com/beyarkay/eskom-calendar-api/issues/new)!

## Support

This project is funded by vibes and good words at the moment. If you like what I do,
or you use the project commercially, please consider supporting me:

- <a href="https://ko-fi.com/beyarkay">
    <img alt="Support on Ko-Fi" src="https://img.shields.io/badge/Ko--Fi-Buy%20me%20a%20coffee!-ff5f5f">
  </a>
- <a href="https://patreon.com/user?u=91320409&utm_medium=clipboard_copy&utm_source=copyLink&utm_campaign=creatorshare_creator&utm_content=join_link">
    <img alt="Support on Patreon" src="https://img.shields.io/badge/Patreon-Recurring%20support%20%E2%9D%A4%EF%B8%8F-red">
  </a>


