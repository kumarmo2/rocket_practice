* Currently using **Option<T>** instead of **Result<T,E>** as I have not been able to design the Errors. There are just way too many
  cases to handle. Need to think about it.

* Currently, the **Dtos** take the ownership of the data, as I think creating dto should be the last step and after a dto is
  created, there should be no processing on the model. Not sure if this is the way to go.