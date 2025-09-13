package models
  
import (  
    "database/sql/driver"  
    "github.com/google/uuid"  
	"fmt"
)  
  
type UserId uuid.UUID  
  
// Scan implements sql.Scanner interface  
func (u *UserId) Scan(value interface{}) error {  
    if value == nil {  
        return nil  
    }  
      
    switch v := value.(type) {  
    case string:  
        parsed, err := uuid.Parse(v)  
        if err != nil {  
            return err  
        }  
        *u = UserId(parsed)  
        return nil  
    case []byte:  
        parsed, err := uuid.ParseBytes(v)  
        if err != nil {  
            return err  
        }  
        *u = UserId(parsed)  
        return nil  
    }  
      
    return fmt.Errorf("cannot scan %T into UserId", value)  
}  
  
// Value implements driver.Valuer interface  
func (u UserId) Value() (driver.Value, error) {  
    return uuid.UUID(u).String(), nil  
}