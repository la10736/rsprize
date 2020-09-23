# Come prendere i dati

In generale 

- urlname `rust-language-milano`
- event_id `273387652`


## Dove prendere

Dai partecipanti: https://secure.meetup.com/meetup_api/console/?path=/:urlname/events/:id/attendance

## Estrema ratio

Dagli RSVP: https://secure.meetup.com/meetup_api/console/?path=/:urlname/events/:event_id/rsvps

La struttura di ritorno e' leggermente diversa

```
    #[derive(Deserialize)]
    struct Container {
        member: Member,
        response: String,
    }
```

## Chiamre servizio 

```
curl -vX GET localhost:8080/prizes\?n=3 \
    --header "Content-Type: application/x-www-form-urlencoded" \
    -d @members.json \
    > winners.json
```