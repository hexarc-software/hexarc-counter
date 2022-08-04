# Hexarc Counter

This is a just-for-fun application for tracking views of web pages and displaying the collected stats using 
custom badges from the [shields](https://shields.io/) project.

## Idea
Firstly this application was developed for tracking the views of a GitHub user profile page and displaying 
the view count via badges from the [shields](https://shields.io/) project. In general the tracker can be
set up either in one or many location and the badge settings can be customized to convey a sensible message.

## Usage
All you need is to set up a tracking pixel in a web page and generate a badge for displaying
the collected stats.

### Setup tracker
Create a tracking image `<img src="TRACKER_SERVER/tracker?name=YOUR_TRACKER_NAME">`,
where `YOUR_TRACKER_NAME` is a unique name for you tracker. Embed this image into
a web page which views you want to track.

### View tracker stats
Create the view stats url for your tracker `TRACKER_SERVER/views?name=YOUR_TRACKER_NAME`.
Generate a badge at [shields](https://shields.io/endpoint) using this view stats url.
If you need a custom label for your tracker stats url you can specify it via `CUSTOM_LABEL`
`TRACKER_SERVER/views?name=YOUR_TRACKER_NAME&label=CUSTOM_LABEL`

## Example
Example how to collect and display the views of a [GitHub profile](https://github.com/shadeglare/shadeglare#readme).
You can use the demo server https://hexarc-counter.herokuapp.com/ for experiments.

## License
MIT © [Hexarc Technologies and its contributors](https://github.com/hexarc-tech)