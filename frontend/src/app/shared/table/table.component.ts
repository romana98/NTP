import {
  AfterViewInit,
  Component,
  EventEmitter,
  Input,
  OnChanges,
  Output,
  SimpleChanges,
  ViewChild
} from '@angular/core';
import {MatTableDataSource} from "@angular/material/table";
import {MatSort} from "@angular/material/sort";
import {MatPaginator} from "@angular/material/paginator";

@Component({
  selector: 'app-table',
  templateUrl: './table.component.html',
  styleUrls: ['./table.component.css']
})
export class TableComponent implements OnChanges, AfterViewInit {

  @ViewChild(MatSort) sort: MatSort | undefined;
  @ViewChild(MatPaginator, {static: true}) paginator: MatPaginator | undefined;

  data = new MatTableDataSource<any>();
  pageSize = 10
  @Input() dataSource: any[] = [];
  @Input() columnsToDisplay: string[] = [];
  @Input() columnsToIterate: string[] = [];
  @Output() Delete = new EventEmitter<number>();
  @Output() Click = new EventEmitter<number>();
  @Output() DoubleClick = new EventEmitter<number>();

  constructor() {

  }

  ngAfterViewInit() {


  }

  ngOnChanges(changes: SimpleChanges): void {
    for (const propName in changes) {
      if (changes.hasOwnProperty(propName)) {
        let vary = this.get(propName);
        vary = changes[propName].currentValue;
      }
      if (propName === 'dataSource') {
        this.data = new MatTableDataSource<any>(this.dataSource);
        // @ts-ignore
        this.data.sort = this.sort;
        // @ts-ignore
        this.data.paginator = this.paginator;
      }
    }
  }

  deleted(id: number) {
    this.Delete.emit(id);
  }

  clicked(id: number) {
    this.Click.emit(id);
  }

  doubleClicked(id: number) {
    this.DoubleClick.emit(id);
  }

  get(element: string): string[] {
    switch (element) {
      case 'dataSource':
        return this.dataSource;
      case 'columnsToDisplay':
        return this.columnsToDisplay;
      default:
        return this.columnsToIterate;
    }
  }
}
