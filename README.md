# WJTB_Show_API
API for getting the shows on the site.

#### /api/playing

Returns information about the currently playing show.

If there is a show playing, it will return this:
```
{
	"name":"The name of the currently playing show.",
	"error":"none",
	"end-time":194837590, // the time that the show ends, in seconds from the start of the current day
}
```

If there is no show playing, it will return this:

```
{
	"name":"",
	"error":"no-show",
	"end-time":19682945, // the time when the next show starts, in seconds from the start of the current day
}
```

#### /api/shows/<day>

where <day> is monday, tuesday, wednesday, thursday, or friday.

This will return a list of all the shows that run on that day.

```
{
"day":"monday",
"shows":
  [
	  {
      "name":"show1",
      "desc":"this is a pretty cool show! maybe listen to it!!!",
      "poster":"/img/poster.png",
      "start_time":193868205, // this is the number of seconds from the start of the day
      "end_time":205295762,
      "is_running":1,
    },
    {
      "name":"show2",
      "desc":"this is also a pretty cool show! maybe listen to it!!!",
      "poster":"/img/poster2.png",
      "start_time":293868205,
      "end_time":21395295762,
      "is_running":0,
    },
  ]
}
```
