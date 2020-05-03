Client queues are the queues which are connected to websocket clients for
real time events/notifications.

#### Rules
 * TTL on queue leve.
 * Max limit on messages per queue.
 * Non Durable i.e will not survive broker restart.
 * Non Exclusive.
 
 ##### TODO
 * Need to run a job that clears the non-existent client queues.(Later on need to think how to remove the job and do
 it in real time.)
