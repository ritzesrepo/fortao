/*
Fortao v0.1
github.com/ritzesrepo/fortao
*/

package main

import (
	"encoding/csv"
	"fmt"
	"math/rand"
	"os"
	"time"

	"github.com/spf13/cobra"
)

func main() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

var rootCmd = &cobra.Command{
	Use:   "fortao",
	Short: "A unix-fortune like CLI app in go.",
	Long:  "A unix-fortune like CLI app in go. Print random fortunes or fortaos from CSV(s)",
}

func init() {
	rootCmd.AddCommand(generateCmd)
}

var generateCmd = &cobra.Command{
	Use:   "print [csv_paths...]",
	Short: "Prints a fortao",
	Long:  "Prints a fortao from one or more CSV files.",
	Args:  cobra.MinimumNArgs(1),
	Run:   printFortao,
}

func printFortao(cmd *cobra.Command, args []string) {
	var fortaoList []string
	for _, path := range args {
		file, err := os.Open(path)
		if err != nil {
			fmt.Printf("Error opening file %s: %v\n", path, err)
			continue
		}
		defer file.Close()
		reader := csv.NewReader(file)
		records, err := reader.ReadAll()
		if err != nil {
			fmt.Printf("Error reading file %s: %v\n", path, err)
			continue
		}
		for _, record := range records {
			for _, value := range record {
				if value != "" {
					fortaoList = append(fortaoList, value)
				}
			}
		}
	}
	if len(fortaoList) > 0 {
		rand.Seed(time.Now().UnixNano())
		randomIndex := rand.Intn(len(fortaoList))
		fmt.Println(fortaoList[randomIndex])
	} else {
		fmt.Println("No valid fortaos found.")
	}
}
