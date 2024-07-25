

# Cat Facts

Launches a simple HTTP simple connector that returns a random cat fact.

```bash
$ fluvio cloud connector create --config cat.yaml
```

Should see topics created:

```bash
$ fluvio topic list
```

and Consume that topic:

```bash
$ fluvio consume cat-facts
```

And show inbound/outbound traffics.

# useless facts

Launches a simple HTTP simple connector that returns a random useless fact.

```bash
$ fluvio cloud connector create --config useless.yaml
 ```

 And consume that topic:

```bash
$ fluvio consume useless-facts
```

# Helsinki Metro

Create MQTT connector:

```bash
fluvio cloud connector create --config conn-hsk.yaml
```

And consume that topic:

```bash
fluvio consume helsinki
```

## Consume Jolt SmartModule

Goto Hub and download Jolt SmartModule.

Consume using jolt:

```bash
$ bash h-jolt.bash
```
