# wprs

A very rough command-line client for WordPress, a toy project I use for scheduling a weekly agenda meeting.

I schedule the post using cron and set the date in the title using the date command:

```
date -d "Next Wednesday" +"%b %d, %Y"
```

The entry looks like:

```
wprs create --publish --title "Tinker Weekly: $(date -d 'Next Wednesday' +'%b %d, %Y')" template.html
```


## Setup

To get it to work you need to:

1. Register an app at: https://developer.wordpress.com/

2. Copy the `wprs.conf-example` to `wprs.conf` and fill in app details.

3. Update blog_id, blog_url, and author to match site you want to publish to.

## Usage

### Authentication

Run: `wprs auth`

It will output a URL that you need to open in browser and authenticate, selecting the site you are granting permission.

After authentication, you were be redirected to the site with `?code=XXXX` param, copy the code and enter to get your auth token.

Update your `wprs.conf` with your token.

### Test Connection

Run: `wprs test`

This will use the token and should retrieve the name of the blog.

### Create Post

Run: `wprs create`

First create a file called: `post-template.html` that contains the HTML for the body of the post, then run command and it will create a new post, by default as a draft.

## TODO

At the moment it probably requires some editing of rust source to get it to be usable for anyone else exact purpose. There is still a lot of work to be done to make it more generic. Titles, publish status, dynamic templates, tags, and other bits and bobs.

